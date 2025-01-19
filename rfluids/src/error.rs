//! Error handling.

use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::percent;
use thiserror::Error;

/// CoolProp internal error.
#[derive(Error, Debug, Clone)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);

/// Error during creation of [`BinaryMix`](crate::substance::BinaryMix).
#[derive(Error, Debug, Clone)]
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
#[derive(Error, Debug, Clone)]
pub enum CustomMixError {
    /// Lengths of the specified collections are invalid.
    #[error("Collections of components and fractions should be of the same length!")]
    InvalidLength,

    /// Some of the specified components are not supported.
    #[error("Only pure substances or refrigerants can be used to create custom mixtures!")]
    InvalidComponent,

    /// Some of the specified fractions are invalid.
    #[error("All of the specified fractions should be exclusive between 0 and 100 %!")]
    InvalidFraction,

    /// The sum of the specified fractions is invalid.
    #[error("The sum of the specified fractions should be equal to 100 %!")]
    InvalidFractionsSum,
}
