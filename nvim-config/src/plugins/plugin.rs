use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("Depency missing: {0}")]
    DependencyMissing(String),

    #[error("Plugin not installed: {0}")]
    NotInstalled(String),

    #[error("Error: {0}")]
    Other(Box<dyn Error>),
}

pub trait Plugin {
    fn setup(&self) -> Result<(), PluginError>;
}
