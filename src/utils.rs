use crate::{mlua, nvim, utils, Result};

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

#[macro_export]
macro_rules! lua_value {
    // Empty table
    ($({})?) => {{
        use $crate::mlua;
        mlua::lua().create_table()?
    }};

    // Process nested tables recursively
    (@process $lua:ident { $($inner_key:expr => $inner_value:tt),* $(,)? }) => {
        Value::Table($lua.create_table_from(
            [ $( ($inner_key.to_string(), lua_value!(@process $lua $inner_value)) ),* ]
        )?)
    };

    // Process scalar values with .into_lua()
    (@process $lua:ident $val:expr) => {
        $val.into_lua(&$lua)?
    };

    // Entry point: start building Lua table
    ($value:tt) => {{
        use $crate::mlua::{self, Value, IntoLua};
        let lua = mlua::lua();

        lua_value!(@process lua $value)
    }};
}
