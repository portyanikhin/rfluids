//! Error handling.

use crate::io::FluidParam;
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::percent;
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
}

/// `CoolProp` internal error.
#[derive(Error, Debug, Clone, Eq, PartialEq)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);

/// Error during creation of [`BinaryMix`](crate::substance::BinaryMix).
#[derive(Error, Debug, Clone, PartialEq)]
pub enum BinaryMixError {
    /// Specified fraction is invalid.
    #[error(
        "Specified fraction ({:?} %) is out of possible range [{:.1}; {:.1}] %!",
        .specified.get::<percent>(),
        .min.get::<percent>(),
        .max.get::<percent>()
    )]
    InvalidFraction {
        /// Specified value.
        specified: Ratio,
        /// Minimum possible value.
        min: Ratio,
        /// Maximum possible value.
        max: Ratio,
    },
}

/// Error during creation of [`CustomMix`](crate::substance::CustomMix).
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum CustomMixError {
    /// The specified components are not enough.
    #[error("At least 2 unique components must be provided!")]
    NotEnoughComponents,

    /// Some of the specified fractions are invalid.
    #[error("All of the specified fractions must be exclusive between 0 and 100 %!")]
    InvalidFraction,

    /// The sum of the specified fractions is invalid.
    #[error("The sum of the specified fractions must be equal to 100 %!")]
    InvalidFractionsSum,
}

/// Error during creation of [`Fluid`](crate::fluid::Fluid)
/// from [`CustomMix`](crate::substance::CustomMix).
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidFromCustomMixError {
    /// Specified custom mixture is not supported.
    #[error("Specified custom mixture is not supported! {0}")]
    Unsupported(#[from] CoolPropError),
}

/// Error during [`Fluid::update`](crate::fluid::Fluid::update)
/// or [`Fluid::in_state`](crate::fluid::Fluid::in_state).
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidStateError {
    /// Specified inputs are invalid.
    #[error("Specified inputs ({0:?}, {1:?}) are invalid!")]
    InvalidInputPair(FluidParam, FluidParam),

    /// Some of the specified input value is infinite or NaN.
    #[error("Input values must be finite!")]
    InvalidInputValue,

    /// Failed to update the fluid state due to unsupported inputs or invalid state.
    #[error("Failed to update the fluid state! {0}")]
    UpdateFailed(#[from] CoolPropError),
}

/// Error during calculation of the
/// [`Fluid`](crate::fluid::Fluid) output parameter value.
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidOutputError {
    /// Specified output parameter is not available.
    #[error("Specified output parameter `{0:?}` is not available!")]
    Unavailable(FluidParam),

    /// Failed to calculate the output parameter value.
    #[error("Failed to calculate the output value of `{0:?}`! {1}")]
    CalculationFailed(FluidParam, CoolPropError),
}
