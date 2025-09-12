pub mod cinnamon;
pub mod leap;
pub mod lsp;
mod lua_plugin;
mod plugin;
pub mod spectre;
pub mod telescope;

use nvim_api_helper::{lua::lua_get_global_path, mlua};

use crate::{
    Result, lua_plugin,
    nvim::{
        self,
        api::{self, opts::OptionOpts},
    },
    nvim_dir,
    nvim_helper::lua_value,
    plugins::{lua_plugin::LuaPlugin, plugin::Plugin},
};

fn setup_colorscheme() -> Result<()> {
    api::command("colorscheme gruvbox")?;
    Ok(())
}

fn setup_native_settings() -> Result<()> {
    nvim::api::set_option("number", true)?;
    nvim::api::set_option("scrolloff", 10)?;
    nvim::api::set_option("tabstop", 4)?;
    nvim::api::set_option("shiftwidth", 4)?;
    nvim::api::set_option("softtabstop", 4)?;
    nvim::api::set_option("expandtab", true)?;

    // Firenvim
    mlua::lua().globals().set(
        "firenvim_config",
        lua_value!({
            "globalSettings" => {
                "alt" => "all",
            },
            "localSettings" => {
                ".*" => {
                    "takeover" => "never",
                },
            },
        }),
    )?;
    if lua_get_global_path::<bool>("started_by_firenvim")? {
        nvim::api::set_option("laststatus", 0)?;
    }

    const ENABLE_NEOVIDE: bool = true;
    if ENABLE_NEOVIDE {
        nvim::api::set_option("guifont", "Hack Nerd Font:h9")?;
        nvim::api::command("let g:neovide_cursor_animation_length = 0.05")?;
        nvim::api::command("let g:neovide_cursor_trail_size = 0.3")?;
        nvim::api::command("let g:neovide_scroll_animation_length = 0.1")?;
        nvim::api::command("let g:cinnamon_disable = 1")?;
    }

    Ok(())
}

pub fn setup_plugins() {
    let plugins: Vec<Box<dyn Plugin>> = vec![
        lua_plugin!("nvim-autopairs"),
        lua_plugin!("term-edit", {
            "prompt_end" => "❯ ",
            "feedkeys_delay" => 200,
        }),
        lua_plugin!("toggleterm", {
            "direction" => "tab",
            "start_in_insert" => false,
        }),
        lua_plugin!("guess-indent"),
        lua_plugin!("dirbuf"),
        Box::new(
            LuaPlugin::<_, ()>::builder("nvim-treesitter.configs")
                .pre_setup(|| {
                    let tree_sitter_dir = nvim_dir().join("tree_sitter");
                    let tree_sitter_dir = tree_sitter_dir.to_str().unwrap();

                    let current_rtp: String =
                        api::get_option_value("runtimepath", &OptionOpts::builder().build())?;
                    let modified_rtp = format!("{tree_sitter_dir},{current_rtp}");

                    api::set_option_value(
                        "runtimepath",
                        modified_rtp,
                        &OptionOpts::builder().build(),
                    )?;
                    Ok(lua_value!({
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
                    }))
                })
                .build(),
        ),
    ];

    for plugin in plugins {
        match plugin.setup() {
            Ok(()) => {}
            Err(plugin::PluginError::NotInstalled(name)) => {
                nvim::print!("Plugin {name} doesn't seem to be installed")
            }
            Err(plugin::PluginError::DependencyMissing(name)) => {
                nvim::print!("Dependency {name} seems to be missing")
            }
            Err(plugin::PluginError::Other(e)) => {
                nvim::print!("Error occured: {e}")
            }
        }
    }

    if let Err(e) = setup_native_settings() {
        nvim::print!("Failed to initialize native settings: {e}");
    };

    if let Err(e) = setup_colorscheme() {
        nvim::print!("Failed to initialize colorscheme: {e}");
    };

    if let Err(e) = leap::setup_leap() {
        nvim::print!("Failed to initialize leap: {e}");
    };

    if let Err(e) = telescope::setup_telescope() {
        nvim::print!("Failed to setup telescope: {e}");
    }

    if let Err(e) = lsp::setup_lsp() {
        nvim::print!("Failed to setup lsp: {e}");
    }

    if let Err(e) = cinnamon::setup_cinnamon() {
        nvim::print!("Failed to setup cinnamon: {e}");
    }
}
