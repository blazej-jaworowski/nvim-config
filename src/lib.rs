mod plugins;
mod keymap;
mod keymap_remapping;

pub use nvim_api_helper as nvim_helper;

use nvim_helper::{
    nvim::{self, Dictionary, Function},
    mlua,
    Result, Error,
};

use std::{
    backtrace::Backtrace,
    env,
    fs::{self, OpenOptions},
    io::Write,
    panic,
    path::PathBuf,
};

fn setup_config(_: ()) {
    nvim::print!("Setting up nvim-config");

    plugins::setup_plugins();
    keymap::setup_keymaps();
}

pub fn nvim_dir() -> PathBuf {
    match env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(".nvim"),
        Err(_) => PathBuf::from("/no_home"),
    }
}

#[nvim::plugin(nvim_oxi = nvim)]
fn nvim_config() -> nvim::Result<Dictionary> {
    let nvim_dir = nvim_dir();

    if let Err(e) = fs::create_dir_all(&nvim_dir) {
        nvim::print!("Failed to create .nvim directory: {e}");
        return Ok(Dictionary::new());
    };

    // Log panic to a file instead to stdout (for obvious reasons)
    panic::set_hook(Box::new(move |_| {
        let panic_path = nvim_dir.join("panic.log");
        let panic_path = match panic_path.to_str() {
            Some(path) => path,
            None => {
                nvim::print!("Invalid panic log path");
                return;
            },
        };

        nvim::print!("Panic occured ({panic_path})");

        let backtrace = Backtrace::capture();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(panic_path)
            .inspect_err(|e| {
                nvim::print!("Failed to open panic log file: {e}");
            })
            .expect("Failed to open panic log file");

        _ = writeln!(file, "Panic occurred:\n{}", backtrace)
            .inspect_err(|e| {
                nvim::print!("Failed to write to panic file: {e}");
            });
    }));

    let mut res = Dictionary::new();

    res.insert("setup", Function::from_fn(setup_config));

    Ok(res)
}
