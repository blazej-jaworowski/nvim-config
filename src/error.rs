use thiserror::Error;
use std::result::Result as StdResult;
use crate::nvim::Error as NvimError;
use crate::nvim::api::Error as NvimApiError;
use crate::mlua::prelude::LuaError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("InvalidType error")]
    InvalidType,

    #[error("Nvim error: {0}")]
    Nvim(#[from] NvimError),

    #[error("NvimApi error: {0}")]
    NvimApi(#[from] NvimApiError),

    #[error("Lua error: {0}")]
    Lua(#[from] LuaError),
}

pub type Result<R> = StdResult<R, Error>;
