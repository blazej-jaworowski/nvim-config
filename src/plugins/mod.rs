mod lua_plugins;

pub mod leap;

pub fn setup_plugins() {
    let _ = leap::setup_leap();
}
