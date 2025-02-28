use std::collections::HashMap;

use nvim_oxi::api::opts::CreateCommandOpts;

use crate::utils;
use crate::Result;
use crate::nvim::api::{self, types::{CommandArgs, CommandNArgs}};
use crate::mlua::{self, Table, Function, Value, IntoLua};

pub fn setup_neoscroll() -> Result<()> {
    let neoscroll: Table = utils::require_plugin("neoscroll")?;
    let lua = mlua::lua();

    let setup_func: Function = neoscroll.get("setup")?;
    _ = setup_func.call::<_, Value>(HashMap::from([
        ("mappings", Value::Table(lua.create_table()?)),
        ("hide_cursor", false.into_lua(lua)?),
    ]))?;

    let scroll_func: Function = neoscroll.get("scroll")?;
    let scroll_func_key = mlua::lua().create_registry_value(scroll_func)?;

    api::create_user_command("Neoscroll", move |args: CommandArgs| {
        let scroll_func: Function = mlua::lua().registry_value(&scroll_func_key).unwrap();

        let [amount,] = args.fargs.as_slice() else {
            return;
        };

        let amount: i64 = amount.parse().unwrap();

        _ = scroll_func.call::<_, Value>((
            Value::Integer(amount),
            Value::Boolean(false),
            Value::Integer(100),
            Value::String(lua.create_string("quadratic").unwrap()),
        ));

    }, &CreateCommandOpts::builder().nargs(CommandNArgs::One).build())?;

    Ok(())
}
