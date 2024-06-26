use mlua::{ExternalError, Lua, LuaSerdeExt, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;
use toml_edit::{value, DocumentMut, ImDocument, Key};

// TODO: Better error messages

pub fn parse<'lua>(lua: &'lua Lua, document: Rc<RefCell<DocumentMut>>) -> Result<mlua::Table<'lua>>
where
    'lua: 'static,
{
    let table = lua.create_table()?;
    table.set("__path", Vec::<String>::new())?;

    let metatable = lua.create_table()?;
    let mt_clone = metatable.clone();

    let index_document_copy = Rc::clone(&document);
    metatable.set(
        "__index",
        lua.create_function(move |lua, (tbl, key): (mlua::Table, mlua::String)| {
            let mut path = tbl.get::<_, Vec<String>>("__path")?;
            path.push(key.to_str()?.to_string());

            let binding = index_document_copy.borrow();
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
                    table.set_metatable(Some(mt_clone.clone()));

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
                        table.set_metatable(Some(mt_clone.clone()));
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

    let newindex_document_copy = Rc::clone(&document);
    metatable.set(
        "__newindex",
        lua.create_function(
            move |_, (tbl, key, value): (mlua::Table, mlua::String, Value)| {
                let mut path = tbl.get::<_, Vec<String>>("__path")?;
                path.push(key.to_str()?.to_string());

                let mut binding = newindex_document_copy.borrow_mut();
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

    metatable.set(
        "__tostring",
        lua.create_function(move |lua, ()| lua.to_value(&document.borrow().to_string()))?,
    )?;

    table.set_metatable(Some(metatable.clone()));

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
                    move |lua: &Lua, (path, selector): (mlua::String, Value)| {
                        let initial_key = Key::new("");

                        let (key, item) = path.to_str().unwrap().split('.').try_fold(
                            (&initial_key, document.as_item()),
                            |acc: (&Key, &toml_edit::Item), next_path| {
                                acc.1.as_table().unwrap().get_key_value(next_path).ok_or(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 1,
                                        name: Some("path".into()),
                                        cause: format!(
                                            "key '{}' does not exist in the TOML. The full path that was provided: {}",
                                            next_path,
                                            path.to_str().unwrap()
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
                                            path.to_str().unwrap()
                                        )
                                        .into_lua_err()
                                        .into(),
                                    }
                                )?
                                .get(num as usize)
                                .ok_or(
                                    mlua::Error::BadArgument {
                                        to: Some("span_of".into()),
                                        pos: 2,
                                        name: Some("selector".into()),
                                        cause: format!(
                                            "array '{}' does not have a value at index {}. The full path that was provided: {}",
                                            key.to_string(),
                                            num,
                                            path.to_str().unwrap()
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
