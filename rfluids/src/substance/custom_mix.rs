use super::{BackendName, Pure};
use crate::{io::FluidTrivialParam::MolarMass, native::AbstractState};
use std::collections::HashMap;

/// `CoolProp` custom mixture.
///
/// # See Also
///
/// - [Custom mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Mixtures.html)
#[derive(Clone, Debug, PartialEq)]
pub enum CustomMix {
    /// Mole-based mixture _(with mole fractions)_.
    #[non_exhaustive]
    MoleBased(HashMap<Pure, f64>),

    /// Mass-based mixture _(with mass fractions)_.
    #[non_exhaustive]
    MassBased(HashMap<Pure, f64>),
}

impl CustomMix {
    /// Creates and returns a new [`CustomMix::MoleBased`] instance.
    ///
    /// # Arguments
    ///
    /// - `components` -- hash map of components and
    ///   their _mole_ fractions **\[dimensionless, from 0 to 1\]**
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// assert!(CustomMix::mole_based(HashMap::from([
    ///     (Pure::Water, 0.8),
    ///     (Pure::Ethanol, 0.2),
    /// ]))
    /// .is_ok());
    ///
    /// assert!(CustomMix::mole_based(HashMap::from([
    ///     (Pure::R32, 0.7),
    ///     (Pure::R125, 0.3),
    /// ]))
    /// .is_ok());
    /// ```
    pub fn mole_based(components: HashMap<Pure, f64>) -> Result<Self, CustomMixError> {
        Self::validate(&components)?;
        Ok(Self::MoleBased(components))
    }

    /// Creates and returns a new [`CustomMix::MassBased`] instance.
    ///
    /// # Arguments
    ///
    /// - `components` -- hash map of components and
    ///   their _mass_ fractions **\[dimensionless, from 0 to 1\]**
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// assert!(CustomMix::mass_based(HashMap::from([
    ///     (Pure::Water, 0.6),
    ///     (Pure::Ethanol, 0.4),
    /// ]))
    /// .is_ok());
    ///
    /// assert!(CustomMix::mass_based(HashMap::from([
    ///     (Pure::R32, 0.5),
    ///     (Pure::R125, 0.5),
    /// ]))
    /// .is_ok());
    /// ```
    pub fn mass_based(components: HashMap<Pure, f64>) -> Result<Self, CustomMixError> {
        Self::validate(&components)?;
        Ok(Self::MassBased(components))
    }

    /// Convert to [`CustomMix::MoleBased`]
    /// _(mass fractions will be converted to mole fractions)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let mole_based_mix = CustomMix::mole_based(HashMap::from([
    ///     (Pure::Water, 0.8),
    ///     (Pure::Ethanol, 0.2),
    /// ]))?;
    /// assert_eq!(mole_based_mix.clone().into_mole_based(), mole_based_mix);
    ///
    /// let mass_based_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::R32, 0.5),
    ///     (Pure::R125, 0.5),
    /// ]))?;
    /// assert_ne!(mass_based_mix.clone().into_mole_based(), mass_based_mix);
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    #[must_use]
    pub fn into_mole_based(self) -> Self {
        match self {
            CustomMix::MassBased(c) => {
                let mut components = c;
                let mut sum = 0.0;
                for component in &mut components {
                    *component.1 /= Self::molar_mass(*component.0);
                    sum += *component.1;
                }
                for component in &mut components {
                    *component.1 /= sum;
                }
                Self::MoleBased(HashMap::from_iter(components))
            }
            CustomMix::MoleBased(_) => self,
        }
    }

    /// Specified components and their fractions.
    #[must_use]
    pub fn components(&self) -> &HashMap<Pure, f64> {
        match self {
            CustomMix::MoleBased(components) | CustomMix::MassBased(components) => components,
        }
    }

    fn validate(components: &HashMap<Pure, f64>) -> Result<(), CustomMixError> {
        if components.len() < 2 {
            return Err(CustomMixError::NotEnoughComponents);
        }
        if components.values().any(|f| !(0.0..=1.0).contains(f)) {
            return Err(CustomMixError::InvalidFraction);
        }
        if (components.values().sum::<f64>() - 1.0).abs() > 1e-6 {
            return Err(CustomMixError::InvalidFractionsSum);
        }
        Ok(())
    }

    fn molar_mass(component: Pure) -> f64 {
        AbstractState::new(component.backend_name(), component.as_ref())
            .unwrap()
            .keyed_output(MolarMass)
            .unwrap()
    }
}

/// Error during creation of [`CustomMix`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CustomMixError {
    /// The specified components are not enough.
    #[error("At least 2 unique components must be provided!")]
    NotEnoughComponents,

    /// Some of the specified fractions are invalid.
    #[error("All of the specified fractions must be exclusive between 0 and 1!")]
    InvalidFraction,

    /// The sum of the specified fractions is invalid.
    #[error("The sum of the specified fractions must be equal to 1!")]
    InvalidFractionsSum,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;
    use rstest::*;

    #[rstest]
    #[case(HashMap::from([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)]))]
    #[case(HashMap::from([(Pure::R32, 0.5), (Pure::R125, 0.5)]))]
    fn new_valid_components(#[case] components: HashMap<Pure, f64>) {
        // When
        let res1 = CustomMix::mole_based(components.clone());
        let res2 = CustomMix::mass_based(components);

        // Then
        assert!(res1.is_ok());
        assert!(res2.is_ok());
    }

    #[rstest]
    #[case(HashMap::from([(Pure::Water, 0.6)]), CustomMixError::NotEnoughComponents)]
    #[case(
            HashMap::from([(Pure::Water, 0.5), (Pure::Water, 0.5)]),
            CustomMixError::NotEnoughComponents
        )]
    #[case(
            HashMap::from([(Pure::R32, -0.5), (Pure::R125, 0.5)]),
            CustomMixError::InvalidFraction
        )]
    #[case(
            HashMap::from([(Pure::R32, 1.5), (Pure::R125, 0.5)]),
            CustomMixError::InvalidFraction
        )]
    #[case(
            HashMap::from([(Pure::R32, 0.4), (Pure::R125, 0.4)]),
            CustomMixError::InvalidFractionsSum
        )]
    fn new_invalid_components(
        #[case] components: HashMap<Pure, f64>,
        #[case] expected: CustomMixError,
    ) {
        // When
        let res1 = CustomMix::mole_based(components.clone());
        let res2 = CustomMix::mass_based(components);

        // Then
        assert_eq!(res1, Err(expected.clone()));
        assert_eq!(res2, Err(expected));
    }

    #[test]
    fn into_mole_based_from_mole_based() {
        // Given
        let sut = CustomMix::mole_based(HashMap::from([(Pure::Water, 0.8), (Pure::Ethanol, 0.2)]))
            .unwrap();

        // When
        let res = sut.clone().into_mole_based();

        // Then
        assert_eq!(res, sut);
        assert!(matches(&res, [("Water", 0.8), ("Ethanol", 0.2)]));
    }

    #[test]
    fn into_mole_based_from_mass_based() {
        // Given
        let sut =
            CustomMix::mass_based(HashMap::from([(Pure::R32, 0.5), (Pure::R125, 0.5)])).unwrap();

        // When
        let res = sut.clone().into_mole_based();

        // Then
        assert_ne!(res, sut);
        assert!(matches(&sut, [("R32", 0.5), ("R125", 0.5)]));
        assert!(matches(
            &res,
            [
                ("R32", 0.697_614_699_375_862_4),
                ("R125", 0.302_385_300_624_137_54)
            ]
        ));
    }

    fn matches(mix: &CustomMix, expected: [(&str, f64); 2]) -> bool {
        mix.components().len() == expected.len()
            && mix
                .components()
                .iter()
                .filter(|component| {
                    expected.iter().any(|exp| {
                        component.0.as_ref() == exp.0 && relative_eq!(*component.1, exp.1)
                    })
                })
                .count()
                == expected.len()
    }
}
