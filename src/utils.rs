use std::rc::Rc;
use crate::{mlua, nvim, utils, Result, Error, lua_value};
use nvim::api::types::CommandArgs;

use mlua::{FromLuaMulti, IntoLuaMulti, FromLua, Function, Table, Value};

pub fn call_lua_func<'lua, A, R>(func: &str, args: A) -> Result<R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    let lua = mlua::lua();
    let lua_func: Function = lua.globals().get(func)?;

    let result = lua_func.call(args);
    match result {
        Ok(ret) => Ok(R::from_lua_multi(ret, lua)?),
        Err(e) => {
            nvim::print!("Lua '{func}' call failed: {e}");
            Err(e.into())
        },
    }
}

pub fn require_plugin<'lua, R>(plugin_name: &str) -> Result<R>
where
R: FromLuaMulti<'lua>
{
    utils::call_lua_func("require", (plugin_name,))
}

pub fn require_get_func<'lua>(plugin_name: &str, func_name: &str) -> Result<Function<'lua>> {
    let plugin: Table = require_plugin(plugin_name)?;
    let func: Function = plugin.get(func_name)?;

    Ok(func)
}

pub fn require_call_func<'lua, A, R>(plugin_name: &str, func_name: &str, args: A) -> Result<R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{
    let func = require_get_func(plugin_name, func_name)?;
    let ret = func.call(args)?;
    Ok(ret)
}

pub fn require_call_setup<'lua, A, R>(plugin_name: &str, args: A) -> Result<R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{
    require_call_func(plugin_name, "setup", args)
}

pub fn wrap_command<F>(command: F) -> impl Fn(CommandArgs)
where
F: Fn(CommandArgs) -> Result<()>,
{
   move |args: CommandArgs| {
        if let Err(e) = command(args) {
            nvim::print!("Command failed: {e}");
        }
    }
}

pub fn lua_get_value_path<'lua, T>(mut table: Table<'lua>, path: &str) -> Result<T>
where
T: FromLua<'lua>
{
    let parts: Vec<&str> = path.split(".").collect();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            return Ok(table.get(*part)?)
        } else {
            table = table.get(*part)?
        }
    }

    Err(Error::InvalidType)
}

pub fn lua_get_global_value_path<'lua, T>(path: &str) -> Result<T>
where
T: FromLua<'lua>
{
    lua_get_value_path(mlua::lua().globals(), path)
}

pub fn lua_registry_named_function(name: &str) -> Rc<dyn Fn() -> Result<()>> {
    let name = name.to_string();
    Rc::new(move || {
        let func: Function = mlua::lua().named_registry_value(&name)?;
        _ = func.call::<_, Value>(lua_value!(()));

        Ok(())
    })
}

#[macro_export]
macro_rules! lua_tuple {
    () => {
        ()
    };

    ($( $value:tt ),* $(,)? ) => {{
        use $crate::lua_value;
        ( $( lua_value!($value) ),* )
    }};
}

#[macro_export]
macro_rules! lua_table {
    () => {{
        use $crate::mlua::{lua, Value};
        Value::Table(lua().create_table()?)
    }};

    ($( $key:expr => $value:tt ),* $(,)? ) => {{
        use $crate::{mlua::{lua, Value}, lua_value};
        Value::Table(lua().create_table_from(
            [ $( (lua_value!($key), lua_value!($value)) ),* ]
        )?)
    }};
}

#[macro_export]
macro_rules! lua_array {
    () => {{
        use $crate::mlua::{lua, Value};
        Value::Table(lua().create_table()?)
    }};

    ($( $value:tt ),* $(,)? ) => {{
        use $crate::{mlua::{lua, Value}, lua_value};
        Value::Table(lua().create_sequence_from(
            [ $( lua_value!($value) ),* ]
        )?)
    }};
}

#[macro_export]
macro_rules! lua_value {
    () => {{
        use $crate::mlua::Value;
        Value::NULL
    }};

    ({ $( $key:expr => $value:tt ),* $(,)? }) => {{
        use $crate::lua_table;
        lua_table!{$( $key => $value ),*}
    }};

    ([ $( $value:tt ),* $(,)? ]) => {{
        use $crate::lua_array;
        lua_array![$( $value ),*]
    }};

    (( $( $value:tt ),* $(,)? )) => {{
        use $crate::lua_tuple;
        lua_tuple!($( $value ),*)
    }};

    ($value:tt) => {{
        use $crate::mlua::{IntoLua, lua};
        $value.into_lua(&lua())?
    }};
}
