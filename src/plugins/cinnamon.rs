use crate::{
    Result,
    mlua::{self, Table, Function, Value},
    nvim,
    nvim_helper::{lua_value, lua_plugins::require_plugin},
};
use crate::keymap_remapping::{NvimAction, NvimKeymap};

use std::rc::Rc;

fn wrap_keys(keys: String) -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(move || {
        let lua = mlua::lua();
        let scroll: Function = lua.named_registry_value("cinnamon_scroll_func")?;
        scroll.call::<_, Value>(keys.to_owned())?;

        Ok(())
    })
}

fn wrap_function(func: Rc<dyn Fn() -> Result<()>>) -> Rc<dyn Fn() -> Result<()>> {
    let func = func.clone();
    Rc::new(move || {
        let func = func.clone();
        let lua = mlua::lua();
        let scroll: Function = lua.named_registry_value("cinnamon_scroll_func")?;
        let lua_func: Function = lua.create_function(move |_, _: ()| {
            if let Err(e) = func() {
                nvim::print!("Action failed: {e}");
            };
            Ok(())
        })?;
        _ = scroll.call::<_, Value>(lua_func)?;

        Ok(())
    })
}

pub fn wrap_action(action: NvimAction) -> Rc<dyn Fn() -> Result<()>> {
    match action {
        NvimAction::Keys(k) => wrap_keys(k),
        NvimAction::Command(c) => wrap_keys(format!(":{c}<CR>")),
        NvimAction::Function(f) => wrap_function(f),
    }
}

// Probably not to be used
#[allow(dead_code)]
pub fn wrap_keymap(keymap: NvimKeymap) -> NvimKeymap {
    keymap.into_iter().map(|(keys, action)| {
        (keys, NvimAction::Function(wrap_action(action)))
    }).collect()
}

pub fn setup_cinnamon() -> Result<()> {
    let cinnamon: Table = require_plugin("cinnamon")?;
    let setup: Function = cinnamon.get("setup")?;

    _ = setup.call::<_, Value>(lua_value!({
        "options" =>{
            "delay" => 3,
            "max_delta" => {
                "line" => 10000,
                "time" => 100,
        },
    }}))?;

    let scroll: Function = cinnamon.get("scroll")?;

    mlua::lua().set_named_registry_value("cinnamon_scroll_func", scroll)?;

    Ok(())
}
