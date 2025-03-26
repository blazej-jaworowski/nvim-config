use std::{collections::HashMap, rc::Rc};
use std::panic::catch_unwind;

use crate::Result;

use crate::nvim::api as api;
use api::{types::Mode, opts::SetKeymapOpts, Buffer};
use crate::nvim;

pub type KeymapFunction = Rc<dyn Fn() -> Result<()>>;

#[derive(Clone)]
pub enum NvimAction {
    Keys(String),
    Command(String),
    Function(KeymapFunction),
}

pub type NvimKeymap = HashMap<String, NvimAction>;

// TODO: Create a separate type and implement Debug instead
#[allow(dead_code)]
pub fn print_keymap(keymap: &NvimKeymap) {
    for (keys, action) in keymap {
        nvim::print!("[{keys}] -> {}\n", match action {
            NvimAction::Keys(k) => format!("[{k}]"),
            NvimAction::Command(c) => c.to_owned(),
            NvimAction::Function(_) => "<lua_function>".to_string(),
        });
    }
}

const ALL_KEYS: [&str; 185] = [
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
    "<C-Space>",
    "<C-Up>", "<C-Down>", "<C-Left>", "<C-Right>",

    // Mouse (basic)
    "<LeftMouse>", "<RightMouse>", "<MiddleMouse>",

    // Keypad (basic examples - add all as needed)
    "<kHome>", "<k0>", "<k1>", "<k2>", "<k3>", "<k4>", "<k5>", "<k6>", "<k7>", "<k8>", "<k9>",
    "<kPlus>", "<kMinus>", "<kMultiply>", "<kDivide>", "<kEnter>", "<kPoint>",

    // Leader keys (placeholders, actual mapping is configuration-dependent)
    "<Leader>", "<LocalLeader>",
];

fn clear_keymap(mode: Mode) -> Result<()> {
    // Iterating over the maps can panic if there is a binding for a mode nvim-oxi doesn't support
    _ = catch_unwind(|| {
        for binding in api::get_keymap(mode) {
            if binding.lhs.starts_with("<Plug>") {
                continue;
            }
            _ = api::del_keymap(mode, &binding.lhs);
        }
    });

    for key in ALL_KEYS {
        api::set_keymap(mode, key, "", &SetKeymapOpts::default())?;
    }

    Ok(())
}

pub fn setup_keymap_clean(mode: Mode, keymap: NvimKeymap) -> Result<()> {
    clear_keymap(mode)?;
    setup_keymap(mode, keymap)?;
    Ok(())
}

pub fn setup_keymap(mode: Mode, keymap: NvimKeymap) -> Result<()> {
    for (binding, action) in keymap.into_iter() {
        let rhs = match action {
            NvimAction::Keys(k) => k,
            NvimAction::Command(cmd) => format!(":{}<CR>", cmd),

            NvimAction::Function(func) => {
                api::set_keymap(
                    mode, &binding, "",
                    &SetKeymapOpts::builder()
                        .silent(true)
                        .callback(move |()| {
                            if let Err(e) = (*func)() {
                                nvim::print!("Keybind function failed: {e}");
                            };
                        })
                        .build()
                )?;
                continue;
            }
        };
        api::set_keymap(
            mode, &binding, &rhs,
            &SetKeymapOpts::builder()
                .silent(true)
                .noremap(true)
                .build()
        )?;
    }

    Ok(())
}

#[allow(dead_code)]
fn clear_buf_keymap(buf: &mut Buffer, mode: Mode) -> Result<()> {
    let buf_handle = buf.handle();

    // This panics no matter what, don't really know why catch_unwind doesn't help
    _ = catch_unwind(|| {
        let mut buf = Buffer::from(buf_handle);

         if let Ok(keymap) = buf.get_keymap(mode) {
             for binding in keymap {
                 if binding.lhs.starts_with("<Plug>") {
                     continue;
                 }
                 _ = buf.del_keymap(mode, &binding.lhs);
             }
         }
    });

    Ok(())
}

pub fn setup_buf_keymap(buf: &mut Buffer, mode: Mode, keymap: NvimKeymap) -> Result<()> {
    for (binding, action) in keymap.into_iter() {
        let rhs = match action {
            NvimAction::Keys(k) => k,
            NvimAction::Command(cmd) => format!(":{}<CR>", cmd),

            NvimAction::Function(func) => {
                buf.set_keymap(
                    mode, &binding, "",
                    &SetKeymapOpts::builder()
                        .silent(true)
                        .callback(move |()| {
                            if let Err(e) = (*func)() {
                                nvim::print!("Keybind function failed: {e}");
                            };
                        })
                        .build()
                )?;
                continue;
            }
        };
        buf.set_keymap(
            mode, &binding, &rhs,
            &SetKeymapOpts::builder()
                .silent(true)
                .noremap(true)
                .build()
        )?;
    }

    Ok(())
}

#[macro_export]
macro_rules! nvim_action {
    ([ $keys:expr ]) => {{
        use $crate::keymap_remapping::NvimAction;
        NvimAction::Keys($keys.into())
    }};

    (! $func:expr ) => {{
        use $crate::keymap_remapping::NvimAction;
        NvimAction::Function($func)
    }};

    (@ $( $action:tt )* ) => {{
        use $crate::plugins::cinnamon::wrap_action;
        use $crate::keymap_remapping::NvimAction;
        NvimAction::Function(wrap_action(nvim_action!($( $action )*)))
    }};

    ($cmd:expr) => {{
        use $crate::keymap_remapping::NvimAction;
        NvimAction::Command($cmd.into())
    }};
}

#[macro_export]
macro_rules! nvim_keymap {
    (@inner ( $str:expr )) => {{
        use $crate::nvim_action;
        ($str.to_string(), nvim_action!([ $str ]))
    }};

    (@inner ( @ $str:expr )) => {{
        use $crate::nvim_action;
        ($str.to_string(), nvim_action!(@ [ $str ]))
    }};

    (@inner ( $str:expr => $( $action:tt )* )) => {{
        use $crate::nvim_action;
        ($str.to_string(), nvim_action!($( $action )*))
    }};

    ($( $item:tt ),* $(,)?) => {{
        use $crate::keymap_remapping::NvimKeymap;
        NvimKeymap::from([
            $( nvim_keymap!(@inner $item) ),*
        ])
    }};
}
