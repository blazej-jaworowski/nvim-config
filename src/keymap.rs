use crate::Result;
use crate::keymap_remapping::{NvimKeymap, setup_keymap};
use crate::nvim::api::types::Mode;
use crate::plugins::leap::leap_cmd;

fn normal_keymap() -> NvimKeymap {
    NvimKeymap::from([
        ("a".to_string(), "a".to_string()),
        ("d".to_string(), "d".to_string()),
        (":".to_string(), ":".to_string()),
        ("f".to_string(), format!(":{}<CR>", leap_cmd())),
    ])
}

pub fn setup_keymaps() -> Result<()> {
    setup_keymap(Mode::Normal, normal_keymap())?;
    Ok(())
}
