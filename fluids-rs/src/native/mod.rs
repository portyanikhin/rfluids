//! Implementation of the [CoolProp](https://coolprop.github.io/CoolProp/) native API.

pub use high_level_api::CoolProp;
pub use low_level_api::AbstractState;

mod common;
mod high_level_api;
mod low_level_api;
