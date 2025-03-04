use crate::keymap_remapping::{NvimKeymap, setup_keymap};
use crate::{nvim_keymap, cmd_call};
use crate::nvim::api::types::Mode;
use crate::nvim;

fn motion_keymap() -> NvimKeymap {
    nvim_keymap![
        // Movement
        ("j" => "h"), ("k" => "j"), ("l" => "k"), (";" => "l"),
        ("K" => cmd_call!("Neoscroll 10")), ("L" => cmd_call!("Neoscroll -10")),
        ("!" => "^"), ("$"),
        ("w"), ("b"), ("e"),
        ("f" => cmd_call!("Leap")),
        ("gg"), ("G"),
        ("<" => "<C-o>"), (">" => "<C-i>"),

        // Window focus
        (" j" => "<C-w>h"),
        (" k" => "<C-w>j"),
        (" l" => "<C-w>k"),
        (" ;" => "<C-w>l"),

        // Mode-change
        ("a"), ("i"), ("A"), ("I"),
        ("o"), ("O"),
        ("v"), ("V"), ("<C-v>"),
        (":"),
        ("<ESC>"),

        // Editing
        ("r"), ("s"), ("x"),
        ("d"), ("y"),
        ("p"), ("P"),

        // Other
        ("<CR>"),
        (" e" => cmd_call!("Dirbuf .")),

        // Undo
        ("u"), ("U" => "<C-r>"),
        (" u" => cmd_call!("UndotreeToggle")),

        // Search
        ("/"),
        ("zf" => cmd_call!("TelescopeCall find_files")),
        ("zd" => cmd_call!("TelescopeCall live_grep")),
    ]
}

pub fn setup_keymaps() {
    let motion_modes = [
        Mode::Normal,
        Mode::Visual,
    ];

    for mode in motion_modes {
        if let Err(e) = setup_keymap(mode, motion_keymap()) {
            nvim::print!("Failed to setup motion keymap for {mode:?}: {e}");
        };
    };
}
