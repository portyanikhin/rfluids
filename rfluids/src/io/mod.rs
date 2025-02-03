//! `CoolProp` inputs/outputs.

pub use fluid_param::*;
pub use humid_air_param::*;
pub use input::*;
pub use input_pair::*;
pub use phase::*;

mod fluid_param;
mod humid_air_param;
mod input;
mod input_pair;
mod phase;

pub(crate) fn try_from<T: TryFrom<u8, Error = strum::ParseError>>(
    value: f64,
) -> Result<T, strum::ParseError> {
    let val = value.trunc();
    if val < f64::from(u8::MIN) || val > f64::from(u8::MAX) {
        return Err(strum::ParseError::VariantNotFound);
    }
    T::try_from(val as u8)
}
