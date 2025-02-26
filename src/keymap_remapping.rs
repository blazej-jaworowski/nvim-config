use std::collections::HashMap;

use crate::Result;

use crate::nvim::api as api;
use api::{types::Mode, opts::SetKeymapOpts};

pub type NvimFunc = String;
pub type NvimKeymap = HashMap<String, NvimFunc>;

const ALL_KEYS: [&str; 203] = [
    // Lowercase letters
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",

    // Uppercase letters
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",

    // Numbers
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",

    // Special characters
    "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "=", "+", "[", "]", "{", "}", ";", ":", "'", "\"", ",", ".", "/", "<", ">", "?", "`", "~",

    // Function keys
    "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>", "<F8>", "<F9>", "<F10>", "<F11>", "<F12>",
    // Add more F keys if needed (up to F24 or higher, depending on terminal support)

    // Navigation keys
    "<Up>", "<Down>", "<Left>", "<Right>", "<Home>", "<End>", "<PageUp>", "<PageDown>",

    // Editing keys
    "<BS>", "<Del>", "<CR>", "<Tab>", "<Esc>", "<Space>",

     // Special characters, escaped where necessary.
    "<Bar>", "<Bslash>",

    // Modifier keys + example character (x).  This needs to be expanded for all relevant characters.
    //  This is the most significant expansion, as it's combinatorial.
    "<C-a>", "<C-b>", "<C-c>", "<C-d>", "<C-e>", "<C-f>", "<C-g>", "<C-h>", "<C-i>", "<C-j>", "<C-k>", "<C-l>", "<C-m>",
    "<C-n>", "<C-o>", "<C-p>", "<C-q>", "<C-r>", "<C-s>", "<C-t>", "<C-u>", "<C-v>", "<C-w>", "<C-x>", "<C-y>", "<C-z>",
    "<C-0>", "<C-1>", "<C-2>", "<C-3>", "<C-4>", "<C-5>", "<C-6>", "<C-7>", "<C-8>", "<C-9>",
    "<C-[>", "<C-]>",  // Control with brackets
    "<M-a>", "<M-b>", "<M-c>",  // ... and so on for all letters, numbers, and some symbols with Meta (Alt)
     "<S-a>", "<S-b>", "<S-c>", // ... and shift
    "<M-x>", "<S-x>", "<C-x>",  // Placeholder - represent ALL combinations
    "<D-x>",  // ... and command (macOS) - This is often the same as Meta.
    "<C-Space>",
    "<S-Up>", "<S-Down>", "<S-Left>", "<S-Right>",  // Shift + Arrow Keys
    "<C-Up>", "<C-Down>", "<C-Left>", "<C-Right>",  // Ctrl + Arrow Keys
    "<M-Up>", "<M-Down>", "<M-Left>", "<M-Right>",    // Meta + Arrow Keys (etc. - many combinations possible)

    // Mouse (basic)
    "<LeftMouse>", "<RightMouse>", "<MiddleMouse>",

    // Keypad (basic examples - add all as needed)
    "<kHome>", "<k0>", "<k1>", "<k2>", "<k3>", "<k4>", "<k5>", "<k6>", "<k7>", "<k8>", "<k9>",
    "<kPlus>", "<kMinus>", "<kMultiply>", "<kDivide>", "<kEnter>", "<kPoint>",

    // Leader keys (placeholders, actual mapping is configuration-dependent)
    "<Leader>", "<LocalLeader>",
];

fn clear_keymap(mode: Mode) -> Result<()> {
    for binding in api::get_keymap(mode) {
        api::del_keymap(mode, &binding.lhs)?;
    }

    for key in ALL_KEYS {
        api::set_keymap(mode, key, "", &SetKeymapOpts::default())?;
    }

    Ok(())
}

pub fn setup_keymap(mode: Mode, keymap: NvimKeymap) -> Result<()> {
    clear_keymap(mode)?;

    for (binding, func) in keymap.into_iter() {
        api::set_keymap(
            mode, &binding, &func,
            &SetKeymapOpts::builder()
                .noremap(true)
                .build()
        )?;
    }

    Ok(())
}
