use crate::Result;

use nvim_oxi as nvim;
use nvim::mlua as mlua;

use mlua::prelude::*;

pub fn call_lua_func<'lua, A, R>(func: &str, args: A) -> Result<R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    let lua = mlua::lua();
    let lua_func: LuaFunction = lua.globals().get(func)?;

    let result = lua_func.call(args);
    match result {
        Ok(ret) => Ok(R::from_lua_multi(ret, lua)?),
        Err(e) => {
            nvim::print!("Lua '{func}' call failed: {e}");
            Err(e.into())
        },
    }
}
