//! Implementation of the [CoolProp](https://coolprop.github.io/CoolProp/) native API.

pub use common::CoolPropError;
pub use enums::*;
pub use high_level_api::CoolProp;
pub use low_level_api::AbstractState;

mod common;
mod enums;
mod high_level_api;
mod low_level_api;
