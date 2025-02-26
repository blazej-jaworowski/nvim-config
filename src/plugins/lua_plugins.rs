use crate::utils;
use crate::Result;

use nvim_oxi as nvim;

use nvim::mlua::FromLua;

pub fn require_lua_plugin<'lua, A>(name: &str) -> Result<A>
where
A: FromLua<'lua>
{
    utils::call_lua_func("require", (name,))
}
