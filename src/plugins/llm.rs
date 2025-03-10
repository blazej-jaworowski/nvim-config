use crate::Result;
use crate::mlua::FromLua;

/*
require("codecompanion").setup({
  adapters = {
    openai = function()
      return require("codecompanion.adapters").extend("openai", {
        env = {
          api_key = "cmd:op read op://personal/OpenAI/credential --no-newline",
        },
      })
    end,
  },
}),
*/

/*
fn openai_adapter<'lua, R>() -> Result<R>
where
    R: FromLua<'lua>,
{
}
*/

pub fn setup_codecompanion() -> Result<()> {
    Ok(())
}
