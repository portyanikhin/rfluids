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
