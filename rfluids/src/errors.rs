//! Error handling.

use thiserror::Error;

/// CoolProp internal error.
#[derive(Error, Debug, Clone)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);
