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
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// assert_eq!(Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(PredefinedMix::R444A.backend_name(), "HEOS");
    /// assert_eq!(BinaryMixKind::MPG.backend_name(), "INCOMP");
    /// assert_eq!(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Water, 0.6),
    ///         (Pure::Ethanol, 0.4),
    ///     ]))?
    ///     .backend_name(),
    ///     "HEOS"
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    fn backend_name(&self) -> &'static str;
}

impl BackendName for Pure {
    fn backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl BackendName for IncompPure {
    fn backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl BackendName for PredefinedMix {
    fn backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl BackendName for BinaryMixKind {
    fn backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl BackendName for CustomMix {
    fn backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use strum::IntoEnumIterator;

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
        let sut = CustomMix::mass_based(HashMap::from([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)]))
            .unwrap();
        assert_eq!(sut.backend_name(), HELMHOLTZ_EOS_BACKEND_NAME);
    }
}
