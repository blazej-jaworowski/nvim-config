use crate::Result;
use crate::utils;
use crate::nvim;
use crate::nvim::api::{self, Buffer, types::Mode, types::{CommandNArgs, CommandArgs}, opts::CreateCommandOpts};
use crate::mlua::{self, Table, Function, Value, prelude::LuaResult, IntoLuaMulti};
use crate::{lua_value, nvim_keymap};
use crate::keymap_remapping::{NvimKeymap, setup_buf_keymap};

fn lsp_define_command_func<'lua, A, F>(command: &str, path: &str, f: F) -> Result<()>
where
A: IntoLuaMulti<'lua>,
F: Fn(CommandArgs) -> Result<A> + 'static,
{
    let lua = mlua::lua();

    let mut obj: Table = lua.globals();
    let mut func: Option<Function> = None;
    let parts: Vec<&str> = path.split(".").collect();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            func = Some(obj.get(*part)?);
        } else {
            obj = obj.get(*part)?;
        }
    }

    let func = func.unwrap();
    let func_key = lua.create_registry_value(func)?;

    api::create_user_command(
        command,
        utils::wrap_command(move |args| -> Result<()> {
            let func: Function = mlua::lua().registry_value(&func_key)?;

            let args = f(args)?;

            _ = func.call::<_, Value>(args)?;
            
            Ok(())
        }),
        &CreateCommandOpts::builder()
            .nargs(CommandNArgs::Zero)
            .range(api::types::CommandRange::WholeFile)
            .build(),
    )?;
    
    Ok(())
}

fn lsp_define_command(command: &str, path: &str) -> Result<()> {
    lsp_define_command_func(command, path, |_| { Ok(()) })
}

fn lsp_define_commands() -> Result<()> {
    lsp_define_command("LspPeekDiagnostic", "vim.diagnostic.open_float")?;
    lsp_define_command("LspDiagnosticList", "vim.diagnostic.setloclist")?;
    lsp_define_command("LspGotoDefinition", "vim.lsp.buf.definition")?;
    lsp_define_command("LspGotoDeclaration", "vim.lsp.buf.declaration")?;
    lsp_define_command("LspGotoImplementation", "vim.lsp.buf.implementation")?;
    lsp_define_command("LspGotoTypeDefinition", "vim.lsp.buf.type_definition")?;
    lsp_define_command("LspGotoReferences", "vim.lsp.buf.references")?;
    lsp_define_command("LspHover", "vim.lsp.buf.hover")?;
    lsp_define_command("LspSignatureHelp", "vim.lsp.buf.signature_help")?;
    lsp_define_command("LspRename", "vim.lsp.buf.rename")?;
    lsp_define_command("LspCodeAction", "vim.lsp.buf.code_action")?;
    lsp_define_command_func("LspFormat", "vim.lsp.buf.format", |args| {
        let range = match args.range {
            0 => Value::Nil,
            _ => lua_value!({
                "start" => [args.line1, 0usize],
                "end" => [args.line2, 0usize],
            }),
        };
        Ok(
            lua_value!({
                "async" => true,
                "range" => range,
            })
        )
    })?;
    Ok(())
}

fn lsp_setup_keymap() -> Result<()> {
    let normal_keymap = nvim_keymap!(
        ("gd" => "LspGotoDefinition"),
        (" q" => "LspDiagnosticList"),
    );
    setup_buf_keymap(&mut Buffer::current(), Mode::Normal, normal_keymap)?;
    Ok(())
}

fn setup_rust(lspconfig: &Table) -> Result<()> {
    let rust_analyzer: Table = lspconfig.get("rust_analyzer")?;
    let setup: Function = rust_analyzer.get("setup")?;

    let blink_cmp_capabilities: Table = utils::require_call_func(
        "blink.cmp",
        "get_lsp_capabilities", lua_value!({})
    )?;

    lsp_define_commands()?;
    let on_attach = mlua::lua().create_function(|_: &mlua::Lua, _: ()| -> LuaResult<()> {
        _ = lsp_setup_keymap().inspect_err(|e| {
            nvim::print!("Error while setting up lsp keymap: {e}");
        });
        Ok(())
    })?;

    setup.call::<_, Value>(lua_value!({
        "capabilities" => blink_cmp_capabilities,
        "on_attach" => on_attach,
    }))?;
    
    Ok(())
}

pub fn setup_lsp() -> Result<()> {
    let lspconfig: Table = utils::require_plugin("lspconfig")?;

    setup_rust(&lspconfig).inspect_err(|_| {
        nvim::print!("Failed to set up rust lsp");
    })?;

    Ok(())
}
