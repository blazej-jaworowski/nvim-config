mod error;
mod utils;
mod plugins;
mod keymap;
mod keymap_remapping;

use error::{Result, Error};

use nvim_oxi as nvim;
use nvim::mlua as mlua;

use nvim::{Dictionary, Function};

fn setup_config(_: ()) {
    nvim::print!("Setting up nvim-config");

    plugins::setup_plugins();
    _ = keymap::setup_keymaps();
}

#[nvim::plugin]
fn libnvim_config() -> nvim::Result<Dictionary> {
    let mut res = Dictionary::new();

    res.insert("setup", Function::from_fn(setup_config));

    Ok(res)
}
