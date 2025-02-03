use super::{BinaryMixKind, CustomMix, IncompPure, PredefinedMix, Pure};

const HELMHOLTZ_EOS_BACKEND_NAME: &str = "HEOS";
const INCOMP_BACKEND_NAME: &str = "INCOMP";

/// `CoolProp` backend name.
pub trait BackendName {
    /// Returns `CoolProp` backend name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::*;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// assert_eq!(Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(PredefinedMix::R444A.backend_name(), "HEOS");
    /// assert_eq!(BinaryMixKind::MPG.backend_name(), "INCOMP");
    /// assert_eq!(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Water, Ratio::new::<percent>(60.0)),
    ///         (Pure::Ethanol, Ratio::new::<percent>(40.0)),
    ///     ]))
    ///     .unwrap()
    ///     .backend_name(),
    ///     "HEOS"
    /// );
    /// ```
    fn backend_name(&self) -> &'static str;
}

macro_rules! heos {
    ($value:ty) => {
        impl BackendName for $value {
            fn backend_name(&self) -> &'static str {
                HELMHOLTZ_EOS_BACKEND_NAME
            }
        }
    };
}

macro_rules! incomp {
    ($value:ty) => {
        impl BackendName for $value {
            fn backend_name(&self) -> &'static str {
                INCOMP_BACKEND_NAME
            }
        }
    };
}

heos!(Pure);
incomp!(IncompPure);
heos!(PredefinedMix);
incomp!(BinaryMixKind);
heos!(CustomMix);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use strum::IntoEnumIterator;
    use uom::si::f64::Ratio;
    use uom::si::ratio::percent;

    #[test]
    fn pure_returns_heos() {
        for substance in Pure::iter() {
            assert_eq!(substance.backend_name(), HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn incomp_pure_returns_incomp() {
        for substance in IncompPure::iter() {
            assert_eq!(substance.backend_name(), INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn predefined_mix_returns_heos() {
        for substance in PredefinedMix::iter() {
            assert_eq!(substance.backend_name(), HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn binary_mix_kind_returns_incomp() {
        for substance in BinaryMixKind::iter() {
            assert_eq!(substance.backend_name(), INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn custom_mix_returns_heos() {
        let sut = CustomMix::mass_based(HashMap::from([
            (Pure::Water, Ratio::new::<percent>(60.0)),
            (Pure::Ethanol, Ratio::new::<percent>(40.0)),
        ]))
        .unwrap();
        assert_eq!(sut.backend_name(), HELMHOLTZ_EOS_BACKEND_NAME);
    }
}
