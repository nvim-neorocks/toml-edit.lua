use mlua::{ExternalError, Lua, LuaSerdeExt, Result};
use std::cell::RefCell;
use std::rc::Rc;
use toml_edit::Document;

// TODO: Better error messages

pub fn parse<'lua>(lua: &'lua Lua, document: Rc<RefCell<Document>>) -> Result<mlua::Table<'lua>>
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
                    entry
                        .as_table()
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
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError(
                                "attempt to index '".to_string()
                                    + next_key.as_str()
                                    + "' (a nil value)",
                            )
                        })
                },
            )?;

            match entry {
                toml_edit::Item::Table(_) => {
                    let table = lua.create_table()?;
                    table.set("__path", path)?;
                    table.set_metatable(Some(mt_clone.clone()));

                    Ok(mlua::Value::Table(table))
                }
                toml_edit::Item::Value(value) => match value {
                    toml_edit::Value::String(str) => lua.to_value(&str.value()),
                    toml_edit::Value::Integer(int) => lua.to_value(&int.value()),
                    toml_edit::Value::Float(float) => lua.to_value(&float.value()),
                    toml_edit::Value::Boolean(bool) => lua.to_value(&bool.value()),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        })?,
    )?;

    let newindex_document_copy = Rc::clone(&document);
    metatable.set(
        "__newindex",
        lua.create_function(
            move |_, (tbl, key, value): (mlua::Table, mlua::String, mlua::Value)| {
                let mut path = tbl.get::<_, Vec<String>>("__path")?;
                path.push(key.to_str()?.to_string());

                // TODO: Don't error on an invalid path
                let mut binding = newindex_document_copy.borrow_mut();
                let entry: &mut toml_edit::Item = path.clone().into_iter().try_fold(
                    binding.as_item_mut(),
                    |entry, next_key| {
                        let value_type = entry.type_name();

                        entry
                            .as_table_mut()
                            .ok_or_else(|| {
                                mlua::Error::RuntimeError(
                                    "attempt to index '".to_string()
                                        + next_key.as_str()
                                        + "' (a "
                                        + value_type
                                        + " value)",
                                )
                            })?
                            .get_mut(next_key.as_str())
                            .ok_or_else(|| {
                                mlua::Error::RuntimeError(
                                    "attempt to index '".to_string()
                                        + next_key.as_str()
                                        + "' (a nil value)",
                                )
                            })
                    },
                )?;

                *entry = match value {
                    mlua::Value::Nil => toml_edit::Item::None,
                    mlua::Value::String(str) => toml_edit::value(str.to_str()?.to_string()),
                    mlua::Value::Number(number) => toml_edit::value(
                        lua.from_value::<mlua::Number>(mlua::Value::Number(number))?,
                    ),
                    mlua::Value::Integer(int) => toml_edit::value(
                        lua.from_value::<mlua::Integer>(mlua::Value::Integer(int))?,
                    ),
                    mlua::Value::Boolean(bool) => toml_edit::value(bool),
                    mlua::Value::Table(table) => todo!(),
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

    table.set_metatable(Some(metatable));

    Ok(table)
}

#[mlua::lua_module]
pub fn toml_edit(lua: &'static Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set(
        "parse",
        lua.create_function(|lua, str: mlua::String| {
            let document: Document = match str.to_string_lossy().parse() {
                Ok(document) => document,
                Err(err) => return Err(err.into_lua_err()),
            };

            parse(lua, RefCell::new(document).into())
        })?,
    )?;
    Ok(table)
}
