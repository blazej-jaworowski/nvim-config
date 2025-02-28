use crate::{lua_value, Result, utils};
use crate::nvim::{self, api};
use crate::mlua::Value;

mod lua_plugins;
pub mod leap;
pub mod neoscroll;

fn setup_autopairs() -> Result<()> {
    let plugin = lua_plugins::LuaPlugin::<(), ()>::builder()
        .name("nvim-autopairs")
        .setup_func("setup")
        .build();
    plugin.setup()
}

fn setup_dirbuf() -> Result<()> {
    utils::require_call_setup::<[u8;0], Value>("dirbuf", [])?;
    Ok(())
}

fn setup_colorscheme() -> Result<()> {
    api::command("colorscheme gruvbox")?;
    Ok(())
}

fn setup_tree_sitter() -> Result<()> {
    let tree_sitter_dir = "/home/b.jaworowski/.nvim";

    let current_rtp: String = api::get_option("runtimepath")?;
    let modified_rtp = format!("{tree_sitter_dir},{current_rtp}");

    api::set_option("runtimepath", modified_rtp)?;

    utils::require_call_setup::<_, Value>("nvim-treesitter.configs", lua_value!({
        // "ensure_installed" => [ "c", "python", "lua" ],
        "auto_install" => false,
        "parser_install_dir" => tree_sitter_dir,
        "highlight" => {
            "enable" => true,
        },
        "indent" => {
            "enable" => true,
        },
        "incremental_selection" => {
            "enable" => true,
        },
    }))?;

    Ok(())
}

pub fn setup_plugins() {

    if let Err(e) = setup_colorscheme() {
        nvim::print!("Failed to initialize colorscheme: {e}");
    };

    if let Err(e) = setup_tree_sitter() {
        nvim::print!("Failed to initialize treesitter: {e}");
    };

    if let Err(e) = leap::setup_leap() {
        nvim::print!("Failed to initialize leap: {e}");
    };

    if let Err(e) = neoscroll::setup_neoscroll() {
        nvim::print!("Failed to setup neoscroll: {e}");
    }

    if let Err(e) = setup_dirbuf() {
        nvim::print!("Failed to setup dirbuf: {e}");
    }

    if let Err(e) = setup_autopairs() {
        nvim::print!("Failed to setup autopairs: {e}");
    }
}
