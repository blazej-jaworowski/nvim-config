mod utils;
mod lua_plugins;

use nvim_oxi as nvim;

use nvim::{Dictionary, Function};

fn setup_config(_: ()) {
    nvim::print!("Setting up nvim-config");

    if let Ok(_) = lua_plugins::setup_lua_plugin("lazy") {

    }
}

#[nvim::plugin]
fn libnvim_config() -> nvim::Result<Dictionary> {
    let mut res = Dictionary::new();

    let fun: Function<(), ()> = Function::from_fn(setup_config);

    res.insert("setup", fun);

    Ok(res)
}
