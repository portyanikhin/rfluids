//! Error handling.

use crate::io::FluidParam;
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::percent;
use thiserror::Error;

/// CoolProp internal error.
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
#[derive(Error, Debug, Clone)]
pub enum FluidFromCustomMixError {
    /// Specified custom mixture is not supported.
    #[error("Specified custom mixture is not supported! {0}")]
    Unsupported(#[from] CoolPropError),
}

/// Error during [`Fluid::update`](crate::fluid::Fluid::update).
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidUpdateError {
    /// Specified inputs are invalid.
    #[error("Specified inputs ({0:?}, {1:?}) are not supported!")]
    InvalidInputPair(FluidParam, FluidParam),

    /// Some of the specified input value is infinite or NaN.
    #[error("Input values must be finite!")]
    InvalidInputValue,

    /// Specified fluid state is invalid.
    #[error("Specified fluid state is invalid! {0}")]
    InvalidState(#[from] CoolPropError),
}
