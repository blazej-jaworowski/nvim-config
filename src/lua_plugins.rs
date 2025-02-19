use crate::utils;

use nvim_oxi as nvim;

use nvim::mlua::FromLua;

fn require_lua_plugin<'lua, A>(name: &str) -> utils::LuaResult<A>
where
A: FromLua<'lua>
{
    utils::call_lua_func("require", (name,))
}

pub fn setup_lua_plugin<'lua>(name: &str) -> utils::LuaResult<()> {
    require_lua_plugin::<[u8;0]>(name)?;

    Ok(())
}

pub fn setup_lua_plugin_func<'lua, F, A>(name: &str, setup_func: Option<F>) -> utils::LuaResult<()>
where
A: FromLua<'lua>,
F: Fn(A) -> (),
{
    let setup_obj: A = require_lua_plugin(name)?;

    if let Some(setup_func) = setup_func {
        setup_func(setup_obj);
    }

    Ok(())
}
