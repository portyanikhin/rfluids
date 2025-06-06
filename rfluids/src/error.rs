use crate::{
    fluid::{FluidFromCustomMixError, FluidOutputError, FluidStateError},
    humid_air::{HumidAirOutputError, HumidAirStateError},
    io::AltitudeError,
    native::CoolPropError,
    substance::{BinaryMixError, CustomMixError},
};
use thiserror::Error;

/// Superset of all possible errors that can occur in the library.
#[derive(Error, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// `CoolProp` internal error.
    #[error(transparent)]
    CoolProp(#[from] CoolPropError),

    /// Error during creation of [`BinaryMix`](crate::substance::BinaryMix).
    #[error(transparent)]
    BinaryMix(#[from] BinaryMixError),

    /// Error during creation of [`CustomMix`](crate::substance::CustomMix).
    #[error(transparent)]
    CustomMix(#[from] CustomMixError),

    /// Error during creation of [`Fluid`](crate::fluid::Fluid)
    /// from [`CustomMix`](crate::substance::CustomMix).
    #[error(transparent)]
    FluidFromCustomMix(#[from] FluidFromCustomMixError),

    /// Error during [`Fluid::update`](crate::fluid::Fluid::update)
    /// or [`Fluid::in_state`](crate::fluid::Fluid::in_state).
    #[error(transparent)]
    FluidState(#[from] FluidStateError),

    /// Error during calculation of the
    /// [`Fluid`](crate::fluid::Fluid) output parameter value.
    #[error(transparent)]
    FluidOutput(#[from] FluidOutputError),

    /// Error during [`HumidAirInput::altitude`](crate::io::HumidAirInput::altitude).
    #[error(transparent)]
    Altitude(#[from] AltitudeError),

    /// Error during [`HumidAir::update`](crate::humid_air::HumidAir::update)
    /// or [`HumidAir::in_state`](crate::humid_air::HumidAir::in_state).
    #[error(transparent)]
    HumidAirState(#[from] HumidAirStateError),

    /// Error during calculation of the
    /// [`HumidAir`](crate::humid_air::HumidAir) output parameter value.
    #[error(transparent)]
    HumidAirOutput(#[from] HumidAirOutputError),
}
