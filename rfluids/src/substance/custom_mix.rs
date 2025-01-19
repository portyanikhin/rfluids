use crate::error::CustomMixError;
use crate::substance::{BackendName, RefrigerantCategory, Substance};
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::ratio;
use crate::uom::ConstZero;

/// CoolProp custom mixture.
///
/// # Examples
///
/// Conversion to [`&str`](str):
///
/// ```
/// use rfluids::substance::{CustomMix, Pure};
/// use rfluids::uom::si::f64::Ratio;
/// use rfluids::uom::si::ratio::percent;
///
/// let mix = CustomMix::try_new(
///     [Pure::Water.into(), Pure::Ethanol.into()],
///     [60.0, 40.0].map(Ratio::new::<percent>)
/// ).unwrap();
/// assert_eq!(mix.as_ref(), "Water&Ethanol");
/// ```
///
/// # See also
///
/// - [Custom mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Mixtures.html)
#[derive(Debug, Clone, PartialEq)]
pub struct CustomMix {
    /// Specified components.
    pub components: Vec<Substance>,
    /// Specified fractions of each component.
    pub fractions: Vec<Ratio>,
    repr: String,
}

impl CustomMix {
    /// Creates and returns a new [`CustomMix`] instance
    /// _(only pure substances or refrigerants are supported)_.
    ///
    /// # Args
    ///
    /// - `components` — collection of components.
    /// - `fractions` — collection of fractions of each component.
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::*;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// assert!(CustomMix::try_new(
    ///     [Pure::Water.into(), Pure::Ethanol.into()],
    ///     [60.0, 40.0].map(Ratio::new::<percent>)
    /// )
    /// .is_ok());
    ///
    /// assert!(CustomMix::try_new(
    ///     [Refrigerant::R32.into(), Refrigerant::R125.into()],
    ///     [50.0, 50.0].map(Ratio::new::<percent>)
    /// )
    /// .is_ok());
    ///
    /// assert!(CustomMix::try_new(
    ///     [Refrigerant::R32.into(), Refrigerant::R125.into()],
    ///     [-50.0, 50.0].map(Ratio::new::<percent>)
    /// )
    /// .is_err());
    ///
    /// assert!(CustomMix::try_new(
    ///     [
    ///         PredefinedMix::TypicalNaturalGas.into(),
    ///         BinaryMix::try_new(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))
    ///             .unwrap()
    ///             .into()
    ///     ],
    ///     [50.0, 50.0].map(Ratio::new::<percent>)
    /// )
    /// .is_err());
    /// ```
    ///
    /// # See also
    ///
    /// - [Custom mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Mixtures.html)
    pub fn try_new(
        components: impl IntoIterator<Item = Substance>,
        fractions: impl IntoIterator<Item = Ratio>,
    ) -> Result<Self, CustomMixError> {
        let components: Vec<_> = components.into_iter().collect();
        let fractions: Vec<_> = fractions.into_iter().collect();
        if components.len() != fractions.len() {
            return Err(CustomMixError::InvalidLength);
        }
        if components.iter().any(Self::is_invalid_component) {
            return Err(CustomMixError::InvalidComponent);
        }
        if fractions
            .iter()
            .any(|f| f <= &Ratio::ZERO || f >= &Ratio::new::<ratio>(1.0))
        {
            return Err(CustomMixError::InvalidFraction);
        }
        if (fractions.iter().map(|f| f.value).sum::<f64>() - 1.0).abs() > 1e-6 {
            return Err(CustomMixError::InvalidFractionsSum);
        }
        let repr = components
            .iter()
            .map(|c| c.as_ref())
            .collect::<Vec<_>>()
            .join("&");
        Ok(Self {
            components,
            fractions,
            repr,
        })
    }

    fn is_invalid_component(component: &Substance) -> bool {
        !matches!(component, Substance::Pure(_))
            && !matches!(
                component,
                Substance::Refrigerant(r) if r.category() == RefrigerantCategory::Pure
            )
    }
}

impl BackendName for CustomMix {
    fn backend_name(&self) -> &'static str {
        "HEOS"
    }
}

impl AsRef<str> for CustomMix {
    fn as_ref(&self) -> &str {
        self.repr.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substance::*;
    use crate::uom::si::ratio::percent;
    use rstest::*;

    #[rstest]
    #[case(vec![Pure::Water.into(), Pure::Ethanol.into()], vec![60.0, 40.0])]
    #[case(vec![Refrigerant::R32.into(), Refrigerant::R125.into()], vec![50.0, 50.0])]
    fn try_new_from_valid_components_and_fractions_always_returns_ok(
        #[case] components: Vec<Substance>,
        #[case] fractions: Vec<f64>,
    ) {
        assert!(
            CustomMix::try_new(components, fractions.into_iter().map(Ratio::new::<percent>))
                .is_ok()
        );
    }

    #[rstest]
    #[case(
        vec![Pure::Water.into(), Pure::Ethanol.into(), Pure::Air.into()],
        vec![60.0, 40.0],
        "Collections of components and fractions should be of the same length!"
    )]
    #[case(
        vec![Pure::Water.into(), PredefinedMix::TypicalNaturalGas.into()],
        vec![60.0, 40.0],
        "Only pure substances or refrigerants can be used to create custom mixtures!"
    )]
    #[case(
        vec![Pure::Water.into(), Refrigerant::R500.into()],
        vec![60.0, 40.0],
        "Only pure substances or refrigerants can be used to create custom mixtures!"
    )]
    #[case(
        vec![Refrigerant::R32.into(), Refrigerant::R125.into()],
        vec![-50.0, 50.0],
        "All of the specified fractions should be exclusive between 0 and 100 %!"
    )]
    #[case(
        vec![Refrigerant::R32.into(), Refrigerant::R125.into()],
        vec![150.0, 50.0],
        "All of the specified fractions should be exclusive between 0 and 100 %!"
    )]
    #[case(
        vec![Refrigerant::R32.into(), Refrigerant::R125.into()],
        vec![40.0, 40.0],
        "The sum of the specified fractions should be equal to 100 %!"
    )]
    fn try_new_from_invalid_components_or_fractions_always_returns_err(
        #[case] components: Vec<Substance>,
        #[case] fractions: Vec<f64>,
        #[case] expected: &str,
    ) {
        assert_eq!(
            CustomMix::try_new(components, fractions.into_iter().map(Ratio::new::<percent>))
                .unwrap_err()
                .to_string(),
            expected
        );
    }

    #[test]
    fn backend_name_always_returns_heos() {
        let sut = CustomMix::try_new(
            vec![Pure::Water.into(), Pure::Ethanol.into()],
            vec![60.0, 40.0].into_iter().map(Ratio::new::<percent>),
        )
        .unwrap();
        assert_eq!(sut.backend_name(), "HEOS");
    }

    #[test]
    fn as_ref_always_returns_expected_str() {
        let sut = CustomMix::try_new(
            vec![Refrigerant::R32.into(), Refrigerant::R125.into()],
            vec![50.0, 50.0].into_iter().map(Ratio::new::<percent>),
        )
        .unwrap();
        assert_eq!(sut.as_ref(), "R32&R125");
    }
}
