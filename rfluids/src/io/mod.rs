//! CoolProp inputs/outputs.

pub use fluid_param::*;
pub use humid_air_param::*;
pub use input::*;
pub use input_pair::*;
pub use phase::*;
pub use update_request::*;

mod fluid_param;
mod humid_air_param;
mod input;
mod input_pair;
mod phase;
mod update_request;

pub(crate) fn try_from<T: TryFrom<u8, Error = strum::ParseError>>(
    value: f64,
) -> Result<T, strum::ParseError> {
    let val = value.trunc();
    if val < u8::MIN as f64 || val > u8::MAX as f64 {
        return Err(strum::ParseError::VariantNotFound);
    }
    T::try_from(val as u8)
}
