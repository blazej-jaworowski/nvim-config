use std::collections::HashMap;

use nvim_oxi::api::opts::CreateCommandOpts;
use nvim_oxi::api::types::CommandArgs;

use crate::utils;
use crate::{nvim::{self, api}, mlua, Result};
use crate::mlua::{Function, Value};

pub fn setup_leap() -> Result<()> {
    let leap_func: Function = utils::require_get_func("leap", "leap")?;

    let leap_func_key = mlua::lua().create_registry_value(leap_func)?;

    api::create_user_command("Leap", move |_: CommandArgs| {
        let func: Function = mlua::lua().registry_value(&leap_func_key).unwrap();

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

        _ = func.call::<_, Value>(HashMap::from([
            ("target_windows", focusable_windows)
        ]));
    }, &CreateCommandOpts::default())?;

    Ok(())
}
