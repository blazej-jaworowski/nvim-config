use std::rc::Rc;
use crate::utils;
use crate::Result;
use crate::mlua::{self, Table, Function, Value};
use crate::lua_value;

pub fn neoscroll(amount: i32) -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(move || {
        let func: Function = mlua::lua().named_registry_value("neoscroll_func")?;

        _ = func.call::<_, Value>(lua_value!((
            amount, false, 100, "quadratic",
        )))?;

        Ok(())
    })
}

pub fn setup_neoscroll() -> Result<()> {
    let neoscroll: Table = utils::require_plugin("neoscroll")?;
    let setup_func: Function = neoscroll.get("setup")?;
    _ = setup_func.call::<_, Value>(lua_value!({
        "mappings" => {},
        "hide_cursor" => false,
    }))?;

    let scroll_func: Function = neoscroll.get("scroll")?;

    mlua::lua().set_named_registry_value("neoscroll_func", scroll_func)?;

    Ok(())
}
