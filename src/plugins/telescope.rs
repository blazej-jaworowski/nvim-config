use crate::{
    Result, Error,
    mlua::{self, Table, Function, Value},
    nvim::{
        self,
        api::{
            self,
            types::{CommandNArgs, CommandArgs},
            opts::CreateCommandOpts,
        },
    },
    nvim_helper::{lua_value, lua_plugins::require_plugin},
};

pub fn setup_telescope() -> Result<()> {
    let telescope: Table = require_plugin("telescope")?;

    let setup_func: Function = telescope.get("setup")?;
    _ = setup_func.call::<_, Value>(lua_value!({
        "defaults" => {
            "vimgrep_arguments" => [
                "rg",
                "--color=never",
                "--no-heading",
                "--with-filename",
                "--line-number",
                "--column",
                "--smart-case",
                "--hidden",
                "--glob", "!**/.git/*"
            ],
            "path_display" => [ "truncate" ],
        },
        "extensions" => {
            "fzf" => {
                "fuzzy" => true,
                "override_generic_sorter" => true,
                "override_file_sorter" => true,
                "case_mode" => "smart_case",
            },
        },
    }));

    let load_extension: Function = telescope.get("load_extension")?;
    _ = load_extension.call::<_, Value>("fzf")?;

    let builtin: Table = require_plugin("telescope.builtin")?;
    let builtin_key = mlua::lua().create_registry_value(builtin)?;

    api::create_user_command(
        "TelescopeCall",
        move |args: CommandArgs| -> Result<()> {
            let builtin: Table = mlua::lua().registry_value(&builtin_key)?;

            let [func_name,] = args.fargs.as_slice() else {
                return Err(Error::InvalidType);
            };

            let func: Function = builtin.get(func_name.to_string()).inspect_err(|_| {
                nvim::print!("Invalid telecope func");
            })?;

            _ = func.call::<_, Value>(mlua::lua().create_table()?)?;

            Ok(())
        },
        &CreateCommandOpts::builder().nargs(CommandNArgs::One).build()
    )?;

    Ok(())
}
