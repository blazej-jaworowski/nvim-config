use crate::utils;
use crate::{Result, Error};
use crate::nvim::api::{self, types::{CommandArgs, CommandNArgs}};
use crate::mlua::{self, Table, Function, Value};
use crate::lua_value;

use nvim_oxi::api::opts::CreateCommandOpts;

pub fn setup_neoscroll() -> Result<()> {
    let neoscroll: Table = utils::require_plugin("neoscroll")?;

    let setup_func: Function = neoscroll.get("setup")?;
    _ = setup_func.call::<_, Value>(lua_value!({
        "mappings" => {},
        "hide_cursor" => false,
    }))?;

    let scroll_func: Function = neoscroll.get("scroll")?;
    let scroll_func_key = mlua::lua().create_registry_value(scroll_func)?;

    api::create_user_command(
        "Neoscroll",
        utils::wrap_command(move |args: CommandArgs| -> Result<()> {
            let scroll_func: Function = mlua::lua().registry_value(&scroll_func_key)?;

            let [amount,] = args.fargs.as_slice() else {
                return Err(Error::InvalidType);
            };

            let amount: i64 = amount.parse().map_err(|_| Error::InvalidType)?;

            _ = scroll_func.call::<_, Value>(lua_value!((
                amount, false, 100, "quadratic"
            )))?;

            Ok(())
        }),
        &CreateCommandOpts::builder().nargs(CommandNArgs::One).build()
    )?;

    Ok(())
}
