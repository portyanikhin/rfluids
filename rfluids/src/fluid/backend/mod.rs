//! `CoolProp` backends.
//!
//! This module provides types for specifying `CoolProp` backends,
//! which determine the underlying equation of state or calculation method used
//! for thermophysical property calculations.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`Backend`] -- represents a `CoolProp` backend, either base or tabular
//! - [`BaseBackend`] -- base backends (e.g., `HEOS`, `REFPROP`, `IF97`, etc.)
//! - [`TabularMethod`] -- tabular interpolation methods (e.g., `TTSE`, `BICUBIC`)
//!
//! # Backend Selection
//!
//! By default, [`Fluid`](crate::fluid::Fluid) uses the default backend for each substance.
//! You can override this using [`Fluid::builder`](crate::fluid::Fluid::builder)
//! to specify a custom backend name.
//!
//! For example, to use the `IF97` backend for water:
//!
//! ```
//! use rfluids::prelude::*;
//!
//! let water = Fluid::builder().substance(Pure::Water).with_backend(BaseBackend::If97).build()?;
//!
//! assert_eq!(water.backend().name(), "IF97");
//! # Ok::<(), rfluids::Error>(())
//! ```

mod base;
mod tabular;

use std::borrow::Cow;

pub use base::*;
pub use tabular::*;

/// `CoolProp` backend.
///
/// This enum represents a `CoolProp` backend, which can be either a base backend
/// or a tabular backend that combines a tabular interpolation method with a base backend.
///
/// # Examples
///
/// Creating a base backend:
///
/// ```
/// use rfluids::prelude::*;
///
/// let base = Backend::Base(BaseBackend::Heos);
/// // or
/// let base: Backend = BaseBackend::Heos.into();
/// assert_eq!(base.name(), "HEOS");
/// ```
///
/// Creating a tabular backend:
///
/// ```
/// use rfluids::prelude::*;
///
/// let tabular = Backend::Tabular { base: BaseBackend::Heos, method: TabularMethod::Ttse };
/// // or
/// let tabular = BaseBackend::Heos.with(TabularMethod::Ttse);
/// assert_eq!(tabular.name(), "TTSE&HEOS");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Backend {
    /// Base backend.
    Base(BaseBackend),
    /// Tabular backend.
    Tabular {
        /// The base backend to use for generating the tables.
        base: BaseBackend,
        /// The tabular interpolation method.
        method: TabularMethod,
    },
}

impl Backend {
    /// Returns the backend name as recognized by `CoolProp`.
    ///
    /// For base backends, returns the backend name directly.
    /// For tabular backends, returns the formatted name in the form
    /// `"METHOD&BASE"` (e.g., `"TTSE&HEOS"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let base: Backend = BaseBackend::If97.into();
    /// assert_eq!(base.name(), "IF97");
    ///
    /// let tabular = BaseBackend::Refprop.with(TabularMethod::Bicubic);
    /// assert_eq!(tabular.name(), "BICUBIC&REFPROP");
    /// ```
    #[must_use]
    pub fn name(&self) -> Cow<'static, str> {
        match self {
            Backend::Base(base) => Cow::Borrowed(base.into()),
            Backend::Tabular { method, base } => {
                Cow::Owned(format!("{}&{}", method.as_ref(), base.as_ref()))
            }
        }
    }
}

impl From<BaseBackend> for Backend {
    fn from(value: BaseBackend) -> Self {
        Self::Base(value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{BaseBackend::*, TabularMethod::*, *};

    #[rstest]
    #[case(Heos, "HEOS", false)]
    #[case(Refprop, "REFPROP", false)]
    #[case(Incomp, "INCOMP", false)]
    #[case(If97, "IF97", false)]
    #[case(Srk, "SRK", false)]
    #[case(Pr, "PR", false)]
    #[case(VtPr, "VTPR", false)]
    #[case(PcSaft, "PCSAFT", false)]
    #[case(Heos.with(Ttse), "TTSE&HEOS", true)]
    #[case(Refprop.with(Ttse), "TTSE&REFPROP", true)]
    #[case(Incomp.with(Ttse), "TTSE&INCOMP", true)]
    #[case(If97.with(Ttse), "TTSE&IF97", true)]
    #[case(Srk.with(Ttse), "TTSE&SRK", true)]
    #[case(Pr.with(Ttse), "TTSE&PR", true)]
    #[case(VtPr.with(Ttse), "TTSE&VTPR", true)]
    #[case(PcSaft.with(Ttse), "TTSE&PCSAFT", true)]
    #[case(Heos.with(Bicubic), "BICUBIC&HEOS", true)]
    #[case(Refprop.with(Bicubic), "BICUBIC&REFPROP", true)]
    #[case(Incomp.with(Bicubic), "BICUBIC&INCOMP", true)]
    #[case(If97.with(Bicubic), "BICUBIC&IF97", true)]
    #[case(Srk.with(Bicubic), "BICUBIC&SRK", true)]
    #[case(Pr.with(Bicubic), "BICUBIC&PR", true)]
    #[case(VtPr.with(Bicubic), "BICUBIC&VTPR", true)]
    #[case(PcSaft.with(Bicubic), "BICUBIC&PCSAFT", true)]
    fn name(#[case] sut: impl Into<Backend>, #[case] expected: &str, #[case] is_owned: bool) {
        // Given
        let sut = sut.into();

        // When
        let res = sut.name();

        // Then
        assert_eq!(res, expected);
        assert_eq!(matches!(res, Cow::Owned(_)), is_owned);
    }
}
