use itertools::Itertools;
use mlua::{ExternalError, Lua, LuaSerdeExt, Result, Table, Value};
use std::cell::RefCell;
use std::rc::Rc;
use toml_edit::{DocumentMut, ImDocument, Key};

// TODO: Better error messages

pub fn parse<'lua>(lua: &'lua Lua, document: Rc<RefCell<DocumentMut>>) -> Result<mlua::Table<'lua>>
where
    'lua: 'static,
{
    let table = lua.create_table()?;
    table.set("__path", Vec::<String>::new())?;

    let list_metatable = lua.create_table()?;
    let list_metatable_clone = list_metatable.clone();
    let list_index_document = Rc::clone(&document);
    list_metatable.set(
        "__index",
        lua.create_function(move |lua, (tbl, key): (mlua::Table, mlua::Integer)| {
            let path = tbl.get::<_, Vec<String>>("__path")?;

            let binding = list_index_document.borrow();
            let entry = path.clone().into_iter().try_fold(
                binding.as_item(),
                |entry, next_key: String| {
                    Ok::<_, mlua::Error>(
                        entry
                            .as_table_like()
                            .ok_or_else(|| {
                                mlua::Error::RuntimeError(
                                    "attempt to index '".to_string()
                                        + next_key.as_str()
                                        + "' (a "
                                        + entry.type_name()
                                        + " value)",
                                )
                            })?
                            .get(next_key.as_str())
                            .unwrap_or_else(|| &toml_edit::Item::None),
                    )
                },
            )?;

            match entry {
                toml_edit::Item::Value(value) => match value {
                    toml_edit::Value::Array(array) => match array.get((key - 1) as usize) {
                        Some(array_value) => match array_value {
                            toml_edit::Value::String(str) => Ok(lua.to_value(&str.value())),
                            toml_edit::Value::Integer(int) => Ok(lua.to_value(&int.value())),
                            toml_edit::Value::Float(float) => Ok(lua.to_value(&float.value())),
                            toml_edit::Value::Boolean(bool) => Ok(lua.to_value(&bool.value())),
                            _ => Err(mlua::Error::RuntimeError(format!(
                                "toml-edit: cannot parse {}: {} yet",
                                array_value.type_name(),
                                array_value
                            ))),
                        },
                        None => Ok(Ok(Value::Nil)),
                    },
                    _ => Err(mlua::Error::RuntimeError(format!(
                        "toml-edit: expected value to be an array type {}: {}",
                        value.type_name(),
                        value
                    ))),
                },
                item => Err(mlua::Error::RuntimeError(format!(
                    "toml-edit: expected item to be a value type, got {}: {}",
                    item.type_name(),
                    item
                ))),
            }
        })?,
    )?;

    let list_newindex_document = Rc::clone(&document);
    list_metatable.set(
        "__newindex",
        lua.create_function(
            move |_, (tbl, key, value): (mlua::Table, mlua::Integer, Value)| {
                let path = tbl.get::<_, Vec<String>>("__path")?;

                let mut binding = list_newindex_document.borrow_mut();
                let entry: &mut toml_edit::Item =
                    path.clone()
                        .into_iter()
                        .fold(binding.as_item_mut(), |entry, next_key| {
                            if !entry.is_table_like() {
                                *entry = toml_edit::Item::Table(toml_edit::Table::default());
                            };

                            let entry = entry.as_table_like_mut().unwrap();

                            if entry.get_mut(next_key.as_str()).is_none() {
                                entry.insert(
                                    next_key.as_str(),
                                    toml_edit::Item::Table(toml_edit::Table::default()),
                                );
                            }

                            entry.get_mut(next_key.as_str()).unwrap()
                        });

                match entry {
                    toml_edit::Item::Value(entry_value) => match entry_value {
                        toml_edit::Value::Array(array) => {
                            let toml_value = match value {
                                Value::String(str) => toml_edit::value(str.to_str()?.to_string()),
                                Value::Number(number) => toml_edit::value(
                                    lua.from_value::<mlua::Number>(Value::Number(number))?,
                                ),
                                Value::Integer(int) => toml_edit::value(
                                    lua.from_value::<mlua::Integer>(Value::Integer(int))?,
                                ),
                                Value::Boolean(bool) => toml_edit::value(bool),
                                _ => {
                                    return Err(mlua::Error::MetaMethodTypeError {
                                        method: "__newindex".into(),
                                        type_name: value.type_name(),
                                        message: Some(
                                            "provided type is not allowed in TOML document".into(),
                                        ),
                                    })
                                }
                            };
                            let index = (key - 1) as usize;
                            let array_value_opt = array.get_mut(index);
                            if let Some(array_value) = array_value_opt {
                                *array_value = toml_value.as_value().unwrap().clone();
                            } else {
                                array.insert(index, toml_value.as_value().unwrap());
                            }
                            return Ok(());
                        }
                        _ => {
                            return Err(mlua::Error::MetaMethodTypeError {
                                method: "__newindex".into(),
                                type_name: value.type_name(),
                                message: Some(format!(
                                    "entry value is not a list type in toml document, {}: {}",
                                    entry_value.type_name(),
                                    entry_value
                                )),
                            })
                        }
                    },
                    item => {
                        return Err(mlua::Error::MetaMethodTypeError {
                            method: "__newindex".into(),
                            type_name: value.type_name(),
                            message: Some(format!(
                                "item is not a value type in toml document, {}: {}",
                                item.type_name(),
                                item
                            )),
                        })
                    }
                }
            },
        )?,
    )?;

    let table_metatable = lua.create_table()?;
    let table_metatable_clone = table_metatable.clone();
    let table_index_document = Rc::clone(&document);
    table_metatable.set(
        "__index",
        lua.create_function(move |lua, (tbl, key): (mlua::Table, mlua::String)| {
            let mut path = tbl.get::<_, Vec<String>>("__path")?;
            path.push(key.to_str()?.to_string());

            let binding = table_index_document.borrow();
            let entry = path.clone().into_iter().try_fold(
                binding.as_item(),
                |entry, next_key: String| {
                    Ok::<_, mlua::Error>(
                        entry
                            .as_table_like()
                            .ok_or_else(|| {
                                mlua::Error::RuntimeError(
                                    "attempt to index '".to_string()
                                        + next_key.as_str()
                                        + "' (a "
                                        + entry.type_name()
                                        + " value)",
                                )
                            })?
                            .get(next_key.as_str())
                            .unwrap_or_else(|| &toml_edit::Item::None),
                    )
                },
            )?;

            match entry {
                toml_edit::Item::Table(_) => {
                    let table = lua.create_table()?;
                    table.set("__path", path)?;
                    table.set_metatable(Some(table_metatable_clone.clone()));

                    Ok(Value::Table(table))
                }
                toml_edit::Item::Value(value) => match value {
                    toml_edit::Value::String(str) => lua.to_value(&str.value()),
                    toml_edit::Value::Integer(int) => lua.to_value(&int.value()),
                    toml_edit::Value::Float(float) => lua.to_value(&float.value()),
                    toml_edit::Value::Boolean(bool) => lua.to_value(&bool.value()),
                    toml_edit::Value::InlineTable(_) => {
                        let table = lua.create_table()?;
                        table.set("__path", path)?;
                        table.set_metatable(Some(table_metatable_clone.clone()));
                        Ok(Value::Table(table))
                    }
                    toml_edit::Value::Array(_) => {
                        let table = lua.create_table()?;
                        table.set("__path", path)?;
                        table.set_metatable(Some(list_metatable_clone.clone()));
                        Ok(Value::Table(table))
                    }
                    _ => Err(mlua::Error::RuntimeError(format!(
                        "toml-edit: cannot parse {}: {} yet",
                        value.type_name(),
                        value
                    ))),
                },
                toml_edit::Item::None => Ok(Value::Nil),
                item => Err(mlua::Error::RuntimeError(format!(
                    "toml-edit: cannot parse: '{}'",
                    item
                ))),
            }
        })?,
    )?;

    let table_newindex_document = Rc::clone(&document);
    table_metatable.set(
        "__newindex",
        lua.create_function(
            move |_, (tbl, key, value): (mlua::Table, mlua::String, Value)| {
                let mut path = tbl.get::<_, Vec<String>>("__path")?;
                path.push(key.to_str()?.to_string());

                let mut binding = table_newindex_document.borrow_mut();
                let entry: &mut toml_edit::Item =
                    path.clone()
                        .into_iter()
                        .fold(binding.as_item_mut(), |entry, next_key| {
                            if !entry.is_table_like() {
                                *entry = toml_edit::Item::Table(toml_edit::Table::default());
                            };

                            let entry = entry.as_table_like_mut().unwrap();

                            if entry.get_mut(next_key.as_str()).is_none() {
                                entry.insert(
                                    next_key.as_str(),
                                    toml_edit::Item::Table(toml_edit::Table::default()),
                                );
                            }

                            entry.get_mut(next_key.as_str()).unwrap()
                        });

                *entry = match value {
                    Value::Nil => toml_edit::Item::None,
                    Value::String(str) => toml_edit::value(str.to_str()?.to_string()),
                    Value::Number(number) => {
                        toml_edit::value(lua.from_value::<mlua::Number>(Value::Number(number))?)
                    }
                    Value::Integer(int) => {
                        toml_edit::value(lua.from_value::<mlua::Integer>(Value::Integer(int))?)
                    }
                    Value::Boolean(bool) => toml_edit::value(bool),
                    Value::Table(_table) => {
                        // Update state data within Lua
                        // for pair in tbl.pairs() {
                        //     let (key, value): (mlua::Value, mlua::Value) = pair?;

                        //     table.set(key, value)?;
                        // }

                        // TODO: The previous invocations just updated everything, so make this now
                        // return a table that allows access to those values.
                        toml_edit::Item::Table(toml_edit::Table::default())
                    }
                    _ => {
                        return Err(mlua::Error::MetaMethodTypeError {
                            method: "__newindex".into(),
                            type_name: value.type_name(),
                            message: Some("provided type is not allowed in TOML document".into()),
                        })
                    }
                };

                Ok(())
            },
        )?,
    )?;

    table_metatable.set(
        "__tostring",
        lua.create_function(move |lua, ()| lua.to_value(&document.borrow().to_string()))?,
    )?;

    table.set_metatable(Some(table_metatable.clone()));

    Ok(table)
}

#[mlua::lua_module]
pub fn toml_edit(lua: &'static Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set(
        "parse",
        lua.create_function(|lua, str: mlua::String| {
            let document: DocumentMut = match str.to_string_lossy().parse() {
                Ok(document) => document,
                Err(err) => return Err(err.into_lua_err()),
            };

            parse(lua, RefCell::new(document).into())
        })?,
    )?;
    table.set(
        "parse_as_tbl",
        lua.create_function(|lua, str: mlua::String| {
            let tbl: toml::Table = match str.to_string_lossy().parse() {
                Ok(tbl) => tbl,
                Err(err) => return Err(err.into_lua_err()),
            };

            lua.to_value(&tbl)
        })?,
    )?;
    table.set(
        "parse_spanned",
        lua.create_function(move |lua: &Lua, str: mlua::String| {
            let spanned = lua.create_table()?;

            let document: ImDocument<String> = match str.to_string_lossy().parse() {
                Ok(document) => document,
                Err(err) => return Err(err.into_lua_err()),
            };

            spanned.set(
                "span_of",
                lua.create_function(
                    move |lua: &Lua, (path, selector): (Table, Value)| {
                        let initial_key = Key::new("");
                        let fullpath = path.clone().sequence_values::<String>().map(|val| val.unwrap()).join(".");

                        let (key, item) = path.sequence_values::<String>().try_fold(
                            (&initial_key, document.as_item()),
                            |acc: (&Key, &toml_edit::Item), next_path| {
                                let next_path = next_path?;

                                acc.1.as_table().unwrap().get_key_value(next_path.as_str()).ok_or(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 1,
                                        name: Some("path".into()),
                                        cause: format!(
                                            "key '{}' does not exist in the TOML. The full path that was provided: {}",
                                            next_path,
                                            fullpath
                                        )
                                        .into_lua_err()
                                        .into(),
                                    },
                                )
                            },
                        )?;

                        match selector {
                            Value::String(str) if str == "key" => Ok(key
                                .span()
                                .map(|val| lua.to_value(&val).unwrap())
                                .unwrap_or(Value::Nil)),
                            Value::String(str) if str == "value" => Ok(item
                                .span()
                                .map(|val| lua.to_value(&val).unwrap())
                                .unwrap_or(Value::Nil)),
                            Value::Integer(num) => Ok(
                                item
                                .as_array()
                                .ok_or(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 2,
                                        name: Some("selector".into()),
                                        cause: format!(
                                            "value {} is not an array (required for numerical selectors)",
                                            fullpath
                                        )
                                        .into_lua_err()
                                        .into(),
                                    }
                                )?
                                .get((num - 1) as usize)
                                .ok_or(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 2,
                                        name: Some("selector".into()),
                                        cause: format!(
                                            "array '{}' does not have a value at index {}. The full path that was provided: {}",
                                            key,
                                            num,
                                            fullpath
                                        )
                                        .into_lua_err()
                                        .into(),
                                    }
                                )?
                                .span()
                                .map(|val| lua.to_value(&val).unwrap())
                                .unwrap_or(Value::Nil)),
                            _ => {
                                Err(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 2,
                                        name: Some("selector".into()),
                                        cause: r#"expected one of: "key", "value" or a number"#
                                            .into_lua_err()
                                            .into(),
                                    },
                                )
                            }
                        }
                    },
                )?,
            )?;

            Ok(spanned)
        })?,
    )?;
    Ok(table)
}
