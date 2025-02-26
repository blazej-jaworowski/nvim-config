use std::collections::HashMap;

use nvim_oxi::api::opts::CreateCommandOpts;
use nvim_oxi::api::types::CommandArgs;

use crate::{Result, Error};
use crate::nvim;
use crate::mlua;
use crate::nvim::api;
use crate::mlua::{Function, Value};
use crate::plugins::lua_plugins;
use crate::keymap_remapping::NvimFunc;

pub fn leap_cmd() -> NvimFunc {
    "Leap".to_string()
}

pub fn setup_leap() -> Result<()> {
    let lua = mlua::lua();

    let leap = lua_plugins::require_lua_plugin::<Value>("leap")?;
    let leap = leap.as_table().ok_or(Error::InvalidType)?;

    let func: Function = leap.get("leap")?;

    let leap_func_key = lua.create_registry_value(func)?;

    api::create_user_command(&leap_cmd(), move |_: CommandArgs| {
        let lua = mlua::lua();

        let func: Function = lua.registry_value(&leap_func_key).unwrap();
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
