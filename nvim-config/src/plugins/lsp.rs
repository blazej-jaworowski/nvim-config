use crate::{
    Result,
    nvim::{self, api::{Buffer, types::Mode}},
    mlua::{self, Table, Function, Value, prelude::LuaResult},
    nvim_helper::{
        lua_value,
        lua_plugins::require_plugin,
        lua::lua_get_global_path,
    },
    nvim_keymap,
    keymap_remapping::setup_buf_keymap,
};

use std::rc::Rc;


pub fn lua_registry_named_function(name: &str) -> Rc<dyn Fn() -> Result<()>> {
    let name = name.to_string();
    Rc::new(move || {
        let func: Function = mlua::lua().named_registry_value(&name)?;
        _ = func.call::<_, Value>(());

        Ok(())
    })
}

fn lsp_setup_func(name: &str, path: &str) -> Result<()> {
    let func: Function = lua_get_global_path(path)?;
    mlua::lua().set_named_registry_value(name, func)?;

    Ok(())
}

fn lsp_define_commands() -> Result<()> {
    lsp_setup_func("lsp_peek_diagnostic", "vim.diagnostic.open_float")?;
    lsp_setup_func("lsp_diagnostic_list", "vim.diagnostic.setloclist")?;

    lsp_setup_func("lsp_goto_definition", "vim.lsp.buf.definition")?;
    lsp_setup_func("lsp_goto_declaration", "vim.lsp.buf.declaration")?;
    lsp_setup_func("lsp_goto_implementation", "vim.lsp.buf.implementation")?;
    lsp_setup_func("lsp_goto_type_definition", "vim.lsp.buf.type_definition")?;
    lsp_setup_func("lsp_goto_references", "vim.lsp.buf.references")?;

    lsp_setup_func("lsp_hover", "vim.lsp.buf.hover")?;
    lsp_setup_func("lsp_signature_help", "vim.lsp.buf.signature_help")?;

    lsp_setup_func("lsp_rename", "vim.lsp.buf.rename")?;
    lsp_setup_func("lsp_code_action", "vim.lsp.buf.code_action")?;

    // lsp_setup_func_func("lsp_format", "vim.lsp.buf.format", |args| {
    //     let range = match args.range {
    //         0 => Value::Nil,
    //         _ => lua_value!({
    //             "start" => [args.line1, 0usize],
    //             "end" => [args.line2, 0usize],
    //         }),
    //     };
    //     Ok(
    //         lua_value!({
    //             "async" => true,
    //             "range" => range,
    //         })
    //     )
    // })?;

    Ok(())
}

fn lsp_setup_keymap() -> Result<()> {
    let insert_keymap = nvim_keymap!(
        ("<C-k>" => ! lua_registry_named_function("lsp_hover")),
        ("<C-l>" => ! lua_registry_named_function("lsp_signature_help")),
    );
    let normal_keymap = nvim_keymap!(
        (".d" => @ ! lua_registry_named_function("lsp_goto_definition")),
        (".D" => @ ! lua_registry_named_function("lsp_goto_declaration")),
        (".i" => @ ! lua_registry_named_function("lsp_goto_implementation")),
        (".t" => @ ! lua_registry_named_function("lsp_goto_type_definition")),
        (".r" => @ ! lua_registry_named_function("lsp_goto_references")),

        (".q" => ! lua_registry_named_function("lsp_diagnostic_list")),
        (".," => ! lua_registry_named_function("lsp_peek_diagnostic")),

        (".ca" => ! lua_registry_named_function("lsp_code_action")),
        (".cr" => ! lua_registry_named_function("lsp_rename")),
        // (".cf" => ! lua_registry_named_function("lsp_format")),

        ("<C-k>" => ! lua_registry_named_function("lsp_hover")),
        ("<C-l>" => ! lua_registry_named_function("lsp_signature_help")),
    );
    setup_buf_keymap(&mut Buffer::current(), Mode::Visual, normal_keymap.clone())?;
    setup_buf_keymap(&mut Buffer::current(), Mode::Normal, normal_keymap)?;
    setup_buf_keymap(&mut Buffer::current(), Mode::Insert, insert_keymap)?;
    Ok(())
}

fn blink_cmp_capabilities<'lua>() -> Result<Table<'lua>> {
    let blink_cmp: Table = require_plugin("blink.cmp")?;
    let blink_cmp_setup: Function = blink_cmp.get("setup")?;
    let get_lsp_capabilities: Function = blink_cmp.get("get_lsp_capabilities")?;

    blink_cmp_setup.call::<_, Value>(lua_value!({
        "fuzzy" => {
            "implementation" => "lua", // TODO: package rust implementation
        },
    }))?;

    Ok(get_lsp_capabilities.call(lua_value!({}))?)
}

fn setup_lang(lang: &str, capabilities: &Table, lspconfig: &Table, on_attach: &Function) -> Result<()> {
    let config: Table = lspconfig.get(lang).inspect_err(|_| {
        nvim::print!("Missing lspconfig lang: {lang}");
    })?;
    let setup: Function = config.get("setup")?;

    setup.call::<_, Value>(lua_value!({
        "capabilities" => capabilities,
        "on_attach" => on_attach,
    }))?;
    
    Ok(())
}

pub fn setup_lsp() -> Result<()> {
    let lspconfig: Table = require_plugin("lspconfig")?;
    lsp_define_commands()?;

    let on_attach = mlua::lua().create_function(|_: &mlua::Lua, _: ()| -> LuaResult<()> {
        _ = lsp_setup_keymap().inspect_err(|e| {
            nvim::print!("Error while setting up lsp keymap: {e}");
        });
        Ok(())
    })?;

    let lsp_capabilities = blink_cmp_capabilities()?;

    setup_lang("rust_analyzer", &lsp_capabilities, &lspconfig, &on_attach).inspect_err(|_| {
        nvim::print!("Failed to set up rust_analyzer lsp");
    })?;

    setup_lang("clangd", &lsp_capabilities, &lspconfig, &on_attach).inspect_err(|_| {
        nvim::print!("Failed to set up clangd lsp");
    })?;

    setup_lang("lua_ls", &lsp_capabilities, &lspconfig, &on_attach).inspect_err(|_| {
        nvim::print!("Failed to set up lua_ls lsp");
    })?;

    Ok(())
}
