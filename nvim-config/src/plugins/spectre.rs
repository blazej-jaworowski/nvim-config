use crate::{
    Result,
    mlua::{self, Table, Function, Value},
    nvim_helper::{lua_value, lua_plugins::require_plugin},
};

use std::rc::Rc;

pub fn spectre_toggle() -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(|| {
        let func: Function = mlua::lua().named_registry_value("spectre_toggle_func")?;
        _ = func.call::<_, Value>(());
        Ok(())
    })
}

#[allow(dead_code)]
pub fn spectre_open_visual(select_word: bool) -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(move || {
        let func: Function = mlua::lua().named_registry_value("spectre_open_visual_func")?;
        _ = func.call::<_, Value>(lua_value!({
            "select_word" => select_word,
        }));
        Ok(())
    })
}

pub fn spectre_open_file_search() -> Rc<dyn Fn() -> Result<()>> {
    Rc::new(move || {
        let func: Function = mlua::lua().named_registry_value("spectre_open_file_search_func")?;
        _ = func.call::<_, Value>(());
        Ok(())
    })
}

pub fn setup_spectre() -> Result<()> {
    let lua = mlua::lua();

    let spectre: Table = require_plugin("spectre")?;

    let toggle_func: Function = spectre.get("toggle")?;
    let open_visual_func: Function = spectre.get("open_visual")?;
    let open_file_search_func: Function = spectre.get("open_file_search")?;

    lua.set_named_registry_value("spectre_toggle_func", toggle_func)?;
    lua.set_named_registry_value("spectre_open_visual_func", open_visual_func)?;
    lua.set_named_registry_value("spectre_open_file_search_func", open_file_search_func)?;

    Ok(())
}
