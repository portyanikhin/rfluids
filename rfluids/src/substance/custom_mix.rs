use crate::error::CustomMixError;
use crate::substance::{BackendName, Pure, Refrigerant, RefrigerantCategory};
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::ratio;
use crate::uom::ConstZero;
use std::collections::HashMap;

/// CoolProp custom mixture
/// _(only pure substances and pure refrigerants are supported)_.
///
/// # Examples
///
/// ```
/// use rfluids::substance::{CustomMix, Pure, Refrigerant};
/// use rfluids::uom::si::f64::Ratio;
/// use rfluids::uom::si::ratio::percent;
/// use std::collections::HashMap;
///
/// assert!(CustomMix::try_from(HashMap::from([
///     (Pure::Water.into(), Ratio::new::<percent>(60.0)),
///     (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
/// ]))
/// .is_ok());
///
/// assert!(CustomMix::try_from(HashMap::from([
///     (Refrigerant::R32.into(), Ratio::new::<percent>(50.0)),
///     (Refrigerant::R125.into(), Ratio::new::<percent>(50.0)),
/// ]))
/// .is_ok());
/// ```
///
/// # See also
///
/// - [Custom mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Mixtures.html)
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct CustomMix(pub HashMap<CustomMixComponent, Ratio>);

impl TryFrom<HashMap<CustomMixComponent, Ratio>> for CustomMix {
    type Error = CustomMixError;

    fn try_from(value: HashMap<CustomMixComponent, Ratio>) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(CustomMixError::NotEnoughComponents);
        }
        if value.keys().any(|component| {
            matches!(
                component,
                CustomMixComponent::Refrigerant(r) if r.category() != RefrigerantCategory::Pure
            )
        }) {
            return Err(CustomMixError::InvalidComponent);
        }
        if value
            .values()
            .any(|f| f <= &Ratio::ZERO || f >= &Ratio::new::<ratio>(1.0))
        {
            return Err(CustomMixError::InvalidFraction);
        }
        if (value.values().map(|f| f.value).sum::<f64>() - 1.0).abs() > 1e-6 {
            return Err(CustomMixError::InvalidFractionsSum);
        }
        Ok(Self(value))
    }
}

impl BackendName for CustomMix {
    fn backend_name(&self) -> &'static str {
        "HEOS"
    }
}

/// Custom mixture component _(pure substance or pure refrigerant)_.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CustomMixComponent {
    /// Pure or pseudo-pure substance.
    Pure(Pure),

    /// Refrigerant.
    Refrigerant(Refrigerant),
}

impl AsRef<str> for CustomMixComponent {
    fn as_ref(&self) -> &str {
        match self {
            CustomMixComponent::Pure(pure) => pure.as_ref(),
            CustomMixComponent::Refrigerant(refrigerant) => refrigerant.as_ref(),
        }
    }
}

impl From<Pure> for CustomMixComponent {
    fn from(value: Pure) -> Self {
        Self::Pure(value)
    }
}

impl From<Refrigerant> for CustomMixComponent {
    fn from(value: Refrigerant) -> Self {
        Self::Refrigerant(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod custom_mix {
        use super::*;
        use crate::uom::si::ratio::percent;
        use rstest::*;

        #[rstest]
        #[case(HashMap::from([(Pure::Water.into(), 60.0), (Pure::Ethanol.into(), 40.0)]))]
        #[case(HashMap::from([(Refrigerant::R32.into(), 50.0), (Refrigerant::R125.into(), 50.0)]))]
        fn try_new_from_valid_components_and_fractions_returns_ok(
            #[case] components: HashMap<CustomMixComponent, f64>,
        ) {
            assert!(CustomMix::try_from(HashMap::from_iter(
                components
                    .into_iter()
                    .map(|c| (c.0, Ratio::new::<percent>(c.1)))
            ))
            .is_ok());
        }

        #[rstest]
        #[case(HashMap::from([(Pure::Water.into(), 60.0)]), CustomMixError::NotEnoughComponents)]
        #[case(
            HashMap::from([(Pure::Water.into(), 50.0), (Pure::Water.into(), 50.0)]),
            CustomMixError::NotEnoughComponents
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 50.0), (Refrigerant::R407C.into(), 50.0)]),
            CustomMixError::InvalidComponent
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), -50.0), (Refrigerant::R125.into(), 50.0)]),
            CustomMixError::InvalidFraction
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 150.0), (Refrigerant::R125.into(), 50.0)]),
            CustomMixError::InvalidFraction
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 40.0), (Refrigerant::R125.into(), 40.0)]),
            CustomMixError::InvalidFractionsSum
        )]
        fn try_new_from_invalid_components_or_fractions_returns_err(
            #[case] components: HashMap<CustomMixComponent, f64>,
            #[case] expected: CustomMixError,
        ) {
            assert_eq!(
                CustomMix::try_from(HashMap::from_iter(
                    components
                        .into_iter()
                        .map(|c| (c.0, Ratio::new::<percent>(c.1)))
                ))
                .unwrap_err(),
                expected
            );
        }

        #[test]
        fn backend_name_returns_heos() {
            let sut = CustomMix::try_from(HashMap::from([
                (Pure::Water.into(), Ratio::new::<percent>(60.0)),
                (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
            ]))
            .unwrap();
            assert_eq!(sut.backend_name(), "HEOS");
        }
    }

    mod custom_mix_component {
        use super::*;

        #[test]
        pub fn custom_mix_component_is_transparent() {
            assert_eq!(
                CustomMixComponent::from(Pure::Water).as_ref(),
                Pure::Water.as_ref()
            );
            assert_eq!(
                CustomMixComponent::from(Refrigerant::R32).as_ref(),
                Refrigerant::R32.as_ref()
            );
        }
    }
}
