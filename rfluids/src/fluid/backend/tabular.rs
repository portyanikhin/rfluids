/// Tabular interpolation method.
///
/// Represents the tabular interpolation methods available in `CoolProp`.
/// Tabular methods dramatically accelerate property calculations by using
/// pre-computed gridded data instead of full equation-of-state evaluations.
///
/// Tabular methods are combined with a [`BaseBackend`](crate::fluid::backend::BaseBackend)
/// to create a tabular backend. The tabular data is generated once and cached for subsequent use.
///
/// # Examples
///
/// Using a tabular backend:
///
/// ```
/// use rfluids::prelude::*;
///
/// let water = Fluid::builder()
///     .substance(Pure::Water)
///     .with_backend(BaseBackend::Heos.with(TabularMethod::Ttse))
///     .build()?;
///
/// assert_eq!(water.backend().name(), "TTSE&HEOS");
/// # Ok::<(), rfluids::Error>(())
/// ```
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
///
/// use rfluids::prelude::*;
///
/// assert_eq!(TabularMethod::Ttse.as_ref(), "TTSE");
/// assert_eq!(TabularMethod::from_str("TTSE"), Ok(TabularMethod::Ttse));
/// assert_eq!(TabularMethod::try_from("ttse"), Ok(TabularMethod::Ttse));
/// ```
///
/// # See Also
///
/// - [Tabular Interpolation](https://coolprop.org/coolprop/Tabular.html)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[non_exhaustive]
pub enum TabularMethod {
    /// Tabular Taylor Series Extrapolation.
    ///
    /// Caches the value and its derivatives with respect to a set of two variables
    /// over a regularly _(linearly or logarithmically)_ spaced grid.
    ///
    /// # See Also
    ///
    /// - [TTSE Interpolation](https://coolprop.org/coolprop/Tabular.html#ttse-interpolation)
    #[strum(to_string = "TTSE")]
    Ttse,

    /// Bicubic interpolation.
    ///
    /// Evaluates the values of the output parameter as well as its derivatives at the
    /// corners of a unit square, and these values are used to fit a bicubic surface
    /// over the unit square.
    ///
    /// # See Also
    ///
    /// - [Bicubic Interpolation](https://coolprop.org/coolprop/Tabular.html#bicubic-interpolation)
    #[strum(to_string = "BICUBIC")]
    Bicubic,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{TabularMethod::*, *};

    #[rstest]
    #[case(Ttse, "TTSE")]
    #[case(Bicubic, "BICUBIC")]
    fn as_str(#[case] sut: TabularMethod, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case(vec!["TTSE", "ttse"], Ttse)]
    #[case(vec!["BICUBIC", "bicubic"], Bicubic)]
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: TabularMethod) {
        for s in valid {
            // When
            let res1 = TabularMethod::from_str(s).unwrap();
            let res2 = TabularMethod::try_from(s).unwrap();

            // Then
            assert_eq!(res1, expected);
            assert_eq!(res2, expected);
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str(#[case] invalid: &str) {
        // When
        let res1 = TabularMethod::from_str(invalid);
        let res2 = TabularMethod::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
