use crate::{
    keymap_remapping::{setup_keymap, setup_keymap_clean, NvimKeymap},
    nvim::{self, api::types::Mode},
    nvim_keymap,
    plugins::{
        leap::leap,
        spectre::{spectre_open_file_search, spectre_toggle},
    },
};

fn motion_keymap() -> NvimKeymap {
    nvim_keymap![
        // Movement
        ("j" => ["h"]), ("k" => ["j"]), ("l" => ["k"]), (";" => ["l"]),
        ("K" => @ ["20j"]), ("L" => @ ["20k"]),
        ("!" => @ ["^"]), (@ "$"),
        (@ "w"), (@ "b"), (@ "e"),
        ("f" => @ ! leap()),
        (@ "gg"), (@ "G"),
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
        ("D" => ["dd"]),
        ("Y" => ["yy"]),
        (@ "p"), (@ "P"),
        ("\""),

        // Other
        ("<CR>"),
        ("ze" => "Dirbuf ."),
        ("<C-j>" => "ToggleTerm"),

        // Save etc.
        (" s" => "w"),
        (" a" => "q"),
        (" A" => "q!"),
        (" e" => "e"),
        (" E" => "e!"),

        // Undo
        ("u"), ("U" => ["<C-r>"]),
        (" u" => "UndotreeToggle"),

        // Search
        (@ "/"),
        ("hh" => "TelescopeCall buffers"),
        ("zf" => "TelescopeCall find_files"),
        ("zd" => "TelescopeCall live_grep"),
        ("?" => "TelescopeCall current_buffer_fuzzy_find"),

        ("RR" => ! spectre_toggle()),
        ("Rf" => ! spectre_open_file_search()),
    ]
}

fn terminal_keymap() -> NvimKeymap {
    nvim_keymap! {
        ("<ESC>" => ["<C-\\><C-n>"]),
    }
}

pub fn setup_keymaps() {
    let motion_modes = [Mode::Normal, Mode::Visual];

    let keymap = motion_keymap();
    for mode in motion_modes {
        let keymap = keymap.clone();
        if let Err(e) = setup_keymap_clean(mode, keymap) {
            nvim::print!("Failed to setup motion keymap for {mode:?}: {e}");
        };
    }
    if let Err(e) = setup_keymap(Mode::Terminal, terminal_keymap()) {
        nvim::print!("Failed to setup terminal keymap: {e}");
    };
}
