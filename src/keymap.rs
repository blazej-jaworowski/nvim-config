use crate::keymap_remapping::{setup_keymap, NvimKeymap};
use crate::plugins::leap::leap;
use crate::plugins::neoscroll::neoscroll;
use crate::nvim_keymap;
use crate::nvim::api::types::Mode;
use crate::nvim;

fn motion_keymap() -> NvimKeymap {
    nvim_keymap![
        // Movement
        ("j" => ["h"]), ("k" => ["j"]), ("l" => ["k"]), (";" => ["l"]),
        ("K" => ! neoscroll(10)), ("L" => ! neoscroll(-10)),
        ("!" => ["^"]), ("$"),
        ("w"), ("b"), ("e"),
        ("f" => ! leap()),
        ("gg"), ("G"),
        ("<" => ["<C-o>"]), (">" => ["<C-i>"]),

        // Window focus
        (" j" => ["<C-w>h"]),
        (" k" => ["<C-w>j"]),
        (" l" => ["<C-w>k"]),
        (" ;" => ["<C-w>l"]),

        // Window management
        (" x" => ["<C-w>c"]),
        (" c" => ["<C-w>s"]),
        (" v" => ["<C-w>v"]),

        // Mode-change
        ("a"), ("i"), ("A"), ("I"),
        ("o"), ("O"),
        ("v"), ("V"), ("<C-v>"),
        (":"),
        ("<ESC>"),

        // Editing
        ("r"), ("s"), ("x"),
        ("S"),
        ("d"), ("y"),
        ("D"), ("Y"),
        ("p"), ("P"),

        // Other
        ("<CR>"),
        (" e" => "Dirbuf ."),

        // Undo
        ("u"), ("U" => ["<C-r>"]),
        (" u" => "UndotreeToggle"),

        // Search
        ("/"),
        ("zf" => "TelescopeCall find_files"),
        ("zd" => "TelescopeCall live_grep"),
    ]
}

pub fn setup_keymaps() {
    let motion_modes = [
        Mode::Normal,
        Mode::Visual,
    ];

    let keymap = motion_keymap();
    for mode in motion_modes {
        let keymap = keymap.clone();
        if let Err(e) = setup_keymap(mode, keymap) {
            nvim::print!("Failed to setup motion keymap for {mode:?}: {e}");
        };
    };
}
