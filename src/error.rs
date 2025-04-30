use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum OneDarkError {
    NvimApi(nvim_oxi::api::Error),
    Config(String),
    Lock(String),
    NvimOxi(nvim_oxi::Error),
    Other(String),
}

impl fmt::Display for OneDarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OneDarkError::NvimApi(e) => write!(f, "Neovim API error: {}", e),
            OneDarkError::NvimOxi(e) => write!(f, "Neovim Oxi error: {}", e),
            OneDarkError::Config(e) => write!(f, "Configuration error: {}", e),
            OneDarkError::Lock(e) => write!(f, "Lock error: {}", e),
            OneDarkError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl Error for OneDarkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OneDarkError::NvimApi(e) => Some(e),
            OneDarkError::NvimOxi(e) => Some(e),
            _ => None,
        }
    }
}

impl From<nvim_oxi::Error> for OneDarkError {
    fn from(err: nvim_oxi::Error) -> Self {
        OneDarkError::NvimOxi(err)
    }
}

impl From<nvim_oxi::api::Error> for OneDarkError {
    fn from(err: nvim_oxi::api::Error) -> Self {
        OneDarkError::NvimApi(err)
    }
}

impl From<OneDarkError> for mlua::Error {
    fn from(err: OneDarkError) -> Self {
        mlua::Error::RuntimeError(err.to_string())
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for OneDarkError {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        OneDarkError::Lock(format!("RwLock read error: {}", err))
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for OneDarkError {
    fn from(err: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        OneDarkError::Lock(format!("RwLock write error: {}", err))
    }
}
