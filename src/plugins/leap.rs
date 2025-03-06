use std::rc::Rc;

use crate::utils;
use crate::{nvim::{self, api}, mlua, Result};
use crate::mlua::{Function, Value};
use crate::lua_value;

pub fn leap() -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(|| {
        let func: Function = mlua::lua().named_registry_value("leap_func")?;

        let focusable_windows = api::list_wins()
            .filter_map(|w| {
                match w.get_config() {
                    Err(e) => {
                        nvim::print!("Error: {e}");
                        None
                    },
                    Ok(c) => if c.focusable.unwrap_or(false) {
                        Some(w.handle())
                    } else {
                        None
                    }
                }
            }).collect::<Vec<_>>();

        _ = func.call::<_, Value>(lua_value!({
            "target_windows" => focusable_windows,
        }));

        Ok(())
    })
}

pub fn setup_leap() -> Result<()> {
    let leap_func: Function = utils::require_get_func("leap", "leap")?;
    mlua::lua().set_named_registry_value("leap_func", leap_func)?;
    Ok(())
}
