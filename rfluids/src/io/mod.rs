//! CoolProp inputs/outputs.

pub use fluid_input::*;
pub use fluid_input_pair::*;
pub use fluid_param::*;
pub use humid_air_param::*;
pub use phase::*;

mod fluid_input;
mod fluid_input_pair;
mod fluid_param;
mod humid_air_param;
mod phase;

/// CoolProp keyed input.
pub trait KeyedInput<K> {
    /// Specified key.
    fn key(&self) -> K;

    /// Specified value _(in SI units)_.
    fn si_value(&self) -> f64;
}
