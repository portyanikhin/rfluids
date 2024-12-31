//! Implementation of the [CoolProp](https://coolprop.github.io/CoolProp/) native API.

pub use common::CoolPropError;
pub use high_level_api::CoolProp;

mod common;
mod high_level_api;
