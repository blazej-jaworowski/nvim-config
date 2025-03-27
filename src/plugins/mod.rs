mod lua_plugins;
pub mod leap;
pub mod lsp;
pub mod telescope;
pub mod spectre;
pub mod cinnamon;


use crate::{
    Result,
    nvim_helper::{
        lua_value,
        lua_plugins::require_call_setup,
    },
    nvim_dir,
    nvim::{self, api::{self, opts::OptionOpts}},
};


fn setup_toggleterm() -> Result<()> {
    require_call_setup("toggleterm", lua_value!({
        "direction" => "tab",
        "start_in_insert" => false,
    }))?;
    Ok(())
}

fn setup_termedit() -> Result<()> {
    require_call_setup("term-edit", lua_value!({
        "prompt_end" => "❯ ",
        "feedkeys_delay" => 200,
    }))?;
    Ok(())
}

fn setup_autopairs() -> Result<()> {
    let plugin = lua_plugins::LuaPlugin::<(), ()>::builder()
        .name("nvim-autopairs")
        .setup_func("setup")
        .build();
    plugin.setup()
}

fn setup_dirbuf() -> Result<()> {
    require_call_setup::<[u8;0]>("dirbuf", [])?;
    Ok(())
}

fn setup_colorscheme() -> Result<()> {
    api::command("colorscheme gruvbox")?;
    Ok(())
}

fn setup_tree_sitter() -> Result<()> {
    let tree_sitter_dir = nvim_dir().join("tree_sitter");
    let tree_sitter_dir = tree_sitter_dir.to_str().unwrap();

    let current_rtp: String = api::get_option_value("runtimepath", &OptionOpts::builder().build())?;
    let modified_rtp = format!("{tree_sitter_dir},{current_rtp}");

    api::set_option_value("runtimepath", modified_rtp, &OptionOpts::builder().build())?;

    require_call_setup("nvim-treesitter.configs", lua_value!({
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

fn setup_native_settings() -> Result<()> {
    nvim::api::set_option("number", true)?;
    nvim::api::set_option("scrolloff", 10)?;

    const ENABLE_NEOVIDE: bool = true;
    if ENABLE_NEOVIDE {
        nvim::api::set_option("guifont", "Hack Nerd Font:h9")?;
        nvim::api::command("let g:neovide_cursor_animation_length = 0.05")?;
        nvim::api::command("let g:neovide_cursor_trail_size = 0.3")?;
        nvim::api::command("let g:neovide_scroll_animation_length = 0.1")?;
        nvim::api::command("let g:cinnamon_disable = 0")?;
    }

    Ok(())
}

pub fn setup_plugins() {
    if let Err(e) = setup_native_settings() {
        nvim::print!("Failed to initialize native settings: {e}");
    };

    if let Err(e) = setup_colorscheme() {
        nvim::print!("Failed to initialize colorscheme: {e}");
    };

    if let Err(e) = setup_tree_sitter() {
        nvim::print!("Failed to initialize treesitter: {e}");
    };

    if let Err(e) = leap::setup_leap() {
        nvim::print!("Failed to initialize leap: {e}");
    };

    if let Err(e) = telescope::setup_telescope() {
        nvim::print!("Failed to setup telescope: {e}");
    }

    if let Err(e) = setup_dirbuf() {
        nvim::print!("Failed to setup dirbuf: {e}");
    }

    if let Err(e) = setup_autopairs() {
        nvim::print!("Failed to setup autopairs: {e}");
    }

    if let Err(e) = lsp::setup_lsp() {
        nvim::print!("Failed to setup lsp: {e}");
    }

    if let Err(e) = setup_toggleterm() {
        nvim::print!("Failed to setup toggleterm: {e}");
    }

    if let Err(e) = spectre::setup_spectre() {
        nvim::print!("Failed to setup spectre: {e}");
    }

    if let Err(e) = cinnamon::setup_cinnamon() {
        nvim::print!("Failed to setup cinnamon: {e}");
    }

    if let Err(e) = setup_termedit() {
        nvim::print!("Failed to setup term-edit: {e}");
    }
}
