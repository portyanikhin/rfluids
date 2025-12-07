use super::{Backend, BaseBackend};
use crate::substance::{BinaryMixKind, CustomMix, IncompPure, PredefinedMix, Pure, Substance};

/// Default `CoolProp` backend for a [`Substance`].
pub trait DefaultBackend {
    /// Returns default `CoolProp` backend for this [`Substance`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = Pure::Water;
    /// assert_eq!(water.default_backend().name(), "HEOS");
    ///
    /// let incomp_water = IncompPure::Water;
    /// assert_eq!(incomp_water.default_backend().name(), "INCOMP");
    ///
    /// let r444a = PredefinedMix::R444A;
    /// assert_eq!(r444a.default_backend().name(), "HEOS");
    ///
    /// let propylene_glycol = BinaryMixKind::MPG;
    /// assert_eq!(propylene_glycol.default_backend().name(), "INCOMP");
    ///
    /// let custom_mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])?;
    /// assert_eq!(custom_mix.default_backend().name(), "HEOS");
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn default_backend(&self) -> Backend;
}

impl DefaultBackend for Substance {
    fn default_backend(&self) -> Backend {
        match self {
            Substance::Pure(pure) => pure.default_backend(),
            Substance::IncompPure(incomp_pure) => incomp_pure.default_backend(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.default_backend(),
            Substance::BinaryMix(binary_mix) => binary_mix.kind.default_backend(),
            Substance::CustomMix(mix) => mix.default_backend(),
        }
    }
}

impl DefaultBackend for Pure {
    fn default_backend(&self) -> Backend {
        BaseBackend::Heos.into()
    }
}

impl DefaultBackend for IncompPure {
    fn default_backend(&self) -> Backend {
        BaseBackend::Incomp.into()
    }
}

impl DefaultBackend for PredefinedMix {
    fn default_backend(&self) -> Backend {
        BaseBackend::Heos.into()
    }
}

impl DefaultBackend for BinaryMixKind {
    fn default_backend(&self) -> Backend {
        BaseBackend::Incomp.into()
    }
}

impl DefaultBackend for CustomMix {
    fn default_backend(&self) -> Backend {
        BaseBackend::Heos.into()
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use strum::IntoEnumIterator;

    use super::*;

    #[rstest]
    #[case(Pure::Water, BaseBackend::Heos.into())]
    #[case(IncompPure::Water, BaseBackend::Incomp.into())]
    #[case(PredefinedMix::R444A, BaseBackend::Heos.into())]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), BaseBackend::Incomp.into())]
    #[case(
        CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])
            .unwrap(),
        BaseBackend::Heos.into()
    )]
    fn substance(#[case] value: impl Into<Substance>, #[case] expected: Backend) {
        // Given
        let sut: Substance = value.into();

        // When
        let res = sut.default_backend();

        // Then
        assert_eq!(res, expected);
    }

    #[test]
    fn pure() {
        for substance in Pure::iter() {
            // When
            let res = substance.default_backend();

            // Then
            assert_eq!(res, BaseBackend::Heos.into());
        }
    }

    #[test]
    fn incomp_pure() {
        for substance in IncompPure::iter() {
            // When
            let res = substance.default_backend();

            // Then
            assert_eq!(res, BaseBackend::Incomp.into());
        }
    }

    #[test]
    fn predefined_mix() {
        for substance in PredefinedMix::iter() {
            // When
            let res = substance.default_backend();

            // Then
            assert_eq!(res, BaseBackend::Heos.into());
        }
    }

    #[test]
    fn binary_mix_kind() {
        for substance in BinaryMixKind::iter() {
            // When
            let res = substance.default_backend();

            // Then
            assert_eq!(res, BaseBackend::Incomp.into());
        }
    }

    #[test]
    fn custom_mix() {
        // Given
        let mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)]).unwrap();

        // When
        let res = mix.default_backend();

        // Then
        assert_eq!(res, BaseBackend::Heos.into());
    }
}
