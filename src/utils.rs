use crate::{mlua, nvim, utils, Result};
use nvim::api::types::CommandArgs;

use mlua::{FromLuaMulti, IntoLuaMulti, Function, Table};

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
            [ $( ($key.to_string(), lua_value!($value)) ),* ]
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
