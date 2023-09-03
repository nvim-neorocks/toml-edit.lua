use mlua::{AnyUserData, ExternalError, Lua, LuaSerdeExt, Result};
use toml_edit::Document;

// TODO: Better error messages

pub fn parse<'lua>(lua: &'lua Lua, str: mlua::String<'lua>) -> Result<mlua::Table<'lua>>
where
    'lua: 'static,
{
    let document: Document = match str.to_string_lossy().parse() {
        Ok(document) => document,
        Err(err) => return Err(err.into_lua_err()),
    };

    let table = lua.create_table()?;
    table.set("__entry", AnyUserData::wrap(document.as_item().clone()))?;

    let metatable = lua.create_table()?;
    let mt_clone = metatable.clone();

    metatable.set(
        "__index",
        lua.create_function(move |lua, payload: (mlua::Table, mlua::String)| {
            let item = payload.0.get::<_, AnyUserData>("__entry")?;
            let item = item.borrow::<toml_edit::Item>()?;

            let item = if item.is_table_like() {
                item.as_table_like()
                    .unwrap()
                    .get(payload.1.to_str()?)
                    .expect("Key not found!")
                    .to_owned()
            } else {
                toml_edit::Item::Value(
                    item.as_array()
                        .unwrap()
                        // Subtract one to account for one-based indexing.
                        .get(payload.1.to_str()?.parse::<usize>().unwrap() - 1)
                        .expect("Index out of bounds!")
                        .to_owned(),
                )
            };

            match item {
                toml_edit::Item::Value(ref val) => match val {
                    toml_edit::Value::String(str) => return lua.to_value(str.value()),
                    toml_edit::Value::Integer(int) => return lua.to_value(int.value()),
                    toml_edit::Value::Float(float) => return lua.to_value(float.value()),
                    toml_edit::Value::Boolean(bool) => return lua.to_value(bool.value()),
                    toml_edit::Value::Array(_) => {
                        let ret = lua.create_table()?;
                        ret.set("__entry", AnyUserData::wrap(item))?;
                        ret.set_metatable(Some(mt_clone.clone()));
                        return Ok(mlua::Value::Table(ret));
                    }
                    toml_edit::Value::InlineTable(_) => {
                        let ret = lua.create_table()?;
                        ret.set("__entry", AnyUserData::wrap(item))?;
                        ret.set_metatable(Some(mt_clone.clone()));
                        return Ok(mlua::Value::Table(ret));
                    }
                    _ => unimplemented!(),
                },
                toml_edit::Item::Table(_) => {
                    let ret = lua.create_table()?;
                    ret.set("__entry", AnyUserData::wrap(item))?;
                    ret.set_metatable(Some(mt_clone.clone()));
                    return Ok(mlua::Value::Table(ret));
                }
                _ => unimplemented!(),
            };
        })?,
    )?;

    metatable.set(
        "__newindex",
        lua.create_function(
            move |_, payload: (mlua::Table, mlua::String, mlua::Value)| {
                let binding: &mut AnyUserData = &mut payload.0.get::<_, AnyUserData>("__entry")?;
                let mut item = binding.borrow_mut::<toml_edit::Item>()?;
                let item: &mut toml_edit::Item =
                    item.get_mut(payload.1.to_str()?).expect("Key not found!");
                *item = toml_edit::Item::Value(toml_edit::Value::String(
                    toml_edit::Formatted::new("hello".to_string()),
                ));
                Ok(())
            },
        )?,
    )?;

    metatable.set(
        "__tostring",
        lua.create_function(move |lua, ()| lua.to_value(&document.to_string()))?,
    )?;

    table.set_metatable(Some(metatable));

    Ok(table)
}

#[mlua::lua_module]
pub fn toml_edit(lua: &'static Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set("parse", lua.create_function(parse)?)?;
    Ok(table)
}
