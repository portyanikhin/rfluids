//! `CoolProp` substances.
//!
//! Provides types and functionality for working with `CoolProp` substances.
//!
//! This module defines various substance types that can be used with `CoolProp`,
//! including pure substances and different types of mixtures.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`Substance`] -- an enum representing different types of substances:
//!
//!     - [`Pure`] -- pure or pseudo-pure substances
//!     - [`IncompPure`] -- incompressible pure substances
//!     - [`PredefinedMix`] -- predefined mixtures
//!     - [`BinaryMix`] -- binary mixtures with specified fraction _(mass-based or volume-based)_:
//!
//!         - [`BinaryMixKind`] -- binary mixture kinds
//!
//!     - [`CustomMix`] -- custom mixtures
//!
//! Each substance type can be converted into the [`Substance`] enum
//! using the standard [`From`]/[`Into`] traits.

#![allow(missing_docs, non_camel_case_types)]

mod binary_mix;
mod custom_mix;
mod incomp_pure;
mod predefined_mix;
mod pure;
mod with_backend;

use std::borrow::Cow;

pub use binary_mix::*;
pub use custom_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;
pub use with_backend::*;

use crate::{
    config,
    fluid::backend::{Backend, DefaultBackend},
    io::SubstanceParam,
    native::CoolProp,
};

/// `CoolProp` substance.
///
/// This enum represents different types of substances that can be used in `CoolProp`.
/// Each variant corresponds to a specific type of substance:
///
/// - [`Pure`] -- represents a pure or pseudo-pure substance
/// - [`IncompPure`] -- represents an incompressible pure substance
/// - [`PredefinedMix`] -- represents a predefined mixture
/// - [`BinaryMix`] -- represents an incompressible binary mixture _(mass-based or volume-based)_
/// - [`CustomMix`] -- represents a custom mixture
///
/// `Substance` enum provides [`From`] implementations for each of its variants,
/// allowing for easy conversion from specific substance types to the `Substance` enum.
#[derive(Clone, Debug, PartialEq)]
pub enum Substance {
    /// Pure or pseudo-pure substance.
    Pure(Pure),
    /// Incompressible pure substance.
    IncompPure(IncompPure),
    /// Predefined mixture.
    PredefinedMix(PredefinedMix),
    /// Incompressible binary mixture _(mass-based or volume-based)_.
    BinaryMix(BinaryMix),
    /// Custom mixture.
    CustomMix(CustomMix),
}

impl Substance {
    /// Creates a new [`SubstanceWithBackend`] by cloning this substance and pairing it
    /// with the specified backend.
    ///
    /// This method borrows `self`, so the original substance remains available for use.
    /// If you don't need the original substance after this operation, consider using
    /// [`into_with_backend`](Self::into_with_backend) to avoid the clone.
    ///
    /// # See Also
    ///
    /// - [`Backend`]
    /// - [`SubstanceWithBackend`]
    /// - [`into_with_backend`](Self::into_with_backend)
    /// - [`with_default_backend`](Self::with_default_backend)
    /// - [`into_with_default_backend`](Self::into_with_default_backend)
    #[must_use]
    pub fn with_backend(&self, backend: impl Into<Backend>) -> SubstanceWithBackend {
        SubstanceWithBackend { substance: self.clone(), backend: backend.into() }
    }

    /// Creates a new [`SubstanceWithBackend`] by consuming this substance and pairing it
    /// with the specified backend.
    ///
    /// This method takes ownership of `self`, avoiding the need to clone the substance.
    /// If you need to keep the original substance, use
    /// [`with_backend`](Self::with_backend) instead.
    ///
    /// # See Also
    ///
    /// - [`Backend`]
    /// - [`SubstanceWithBackend`]
    /// - [`with_backend`](Self::with_backend)
    /// - [`with_default_backend`](Self::with_default_backend)
    /// - [`into_with_default_backend`](Self::into_with_default_backend)
    #[must_use]
    pub fn into_with_backend(self, backend: impl Into<Backend>) -> SubstanceWithBackend {
        SubstanceWithBackend { substance: self, backend: backend.into() }
    }

    /// Creates a new [`SubstanceWithBackend`] by cloning this substance and pairing it
    /// with its default backend.
    ///
    /// This method borrows `self`, so the original substance remains available for use.
    /// If you don't need the original substance after this operation, consider using
    /// [`into_with_default_backend`](Self::into_with_default_backend) to avoid the clone.
    ///
    /// # See Also
    ///
    /// - [`Backend`]
    /// - [`DefaultBackend`]
    /// - [`SubstanceWithBackend`]
    /// - [`with_backend`](Self::with_backend)
    /// - [`into_with_backend`](Self::into_with_backend)
    /// - [`into_with_default_backend`](Self::into_with_default_backend)
    #[must_use]
    pub fn with_default_backend(&self) -> SubstanceWithBackend {
        self.with_backend(self.default_backend())
    }

    /// Creates a new [`SubstanceWithBackend`] by consuming this substance and pairing it
    /// with its default backend.
    ///
    /// This method takes ownership of `self`, avoiding the need to clone the substance.
    /// If you need to keep the original substance, use
    /// [`with_default_backend`](Self::with_default_backend) instead.
    ///
    /// # See Also
    ///
    /// - [`Backend`]
    /// - [`DefaultBackend`]
    /// - [`SubstanceWithBackend`]
    /// - [`with_backend`](Self::with_backend)
    /// - [`into_with_backend`](Self::into_with_backend)
    /// - [`with_default_backend`](Self::with_default_backend)
    #[must_use]
    pub fn into_with_default_backend(self) -> SubstanceWithBackend {
        let default_backend = self.default_backend();
        self.into_with_backend(default_backend)
    }

    /// Name.
    ///
    /// # Notes
    ///
    /// - This name is only partially compatible with the `CoolProp` high-level API ([`CoolProp`]):
    ///   it does not include the backend prefix, which is required for [`IncompPure`] and
    ///   [`BinaryMix`]. For API-compatible names, use
    ///   [`SubstanceWithBackend::name`](crate::substance::SubstanceWithBackend::name) instead.
    /// - The name for [`CustomMix`] is constructed based on mole fractions _(i.e., mass fractions
    ///   are converted to mole-based internally)_. Components in [`CustomMix`] are sorted first by
    ///   fraction in descending order, then by name in alphabetical order (for predictable results,
    ///   since the [`HashMap`](std::collections::HashMap) used to store [`CustomMix`] components
    ///   does not guarantee ordering).
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water: Substance = Pure::Water.into();
    /// assert_eq!(water.name(), "Water");
    ///
    /// let incomp_water: Substance = IncompPure::Water.into();
    /// assert_eq!(incomp_water.name(), "Water");
    ///
    /// let r444a: Substance = PredefinedMix::R444A.into();
    /// assert_eq!(r444a.name(), "R444A.mix");
    ///
    /// let propylene_glycol: Substance = BinaryMixKind::MPG.with_fraction(0.4)?.into();
    /// assert_eq!(propylene_glycol.name(), "MPG[0.4]");
    ///
    /// let custom_mix: Substance =
    ///     CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)])?.into();
    ///
    /// // Mass fractions are converted to mole-based internally.
    /// // Components are sorted first by fraction in descending order,
    /// // then by name in alphabetical order
    /// assert_eq!(custom_mix.name(), "Water[0.8]&Ethanol[0.2]");
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    #[must_use]
    pub fn name(&self) -> Cow<'static, str> {
        match self {
            Substance::Pure(pure) => Cow::Borrowed(pure.into()),
            Substance::IncompPure(incomp_pure) => Cow::Borrowed(incomp_pure.into()),
            Substance::PredefinedMix(predefined_mix) => Cow::Borrowed(predefined_mix.into()),
            Substance::BinaryMix(binary_mix) => {
                let name = format!("{}[{}]", binary_mix.kind.as_ref(), binary_mix.fraction);
                Cow::Owned(name)
            }
            Substance::CustomMix(custom_mix) => {
                let mix = custom_mix.clone().into_mole_based();
                let mut components: Vec<_> = mix.components().iter().collect();
                components.sort_by(|left, right| {
                    right.1.total_cmp(left.1).then_with(|| left.0.as_ref().cmp(right.0.as_ref()))
                });
                let name = components
                    .into_iter()
                    .map(|(substance, fraction)| format!("{}[{}]", substance.as_ref(), fraction))
                    .collect::<Vec<_>>()
                    .join("&");
                Cow::Owned(name)
            }
        }
    }

    /// Names of the substance components separated by the `&` symbol
    /// or just a single substance name.
    ///
    /// # Notes
    ///
    /// - This string is compatible with the `CoolProp` low-level API
    ///   ([`AbstractState::new`](crate::native::AbstractState::new))
    /// - For [`Pure`], [`IncompPure`], and [`PredefinedMix`], this string is identical to
    ///   [`name`](Self::name). For [`BinaryMix`] and [`CustomMix`], it differs by omitting the
    ///   fractions (e.g., `"Water&Ethanol"` instead of `"Water[0.8]&Ethanol[0.2]"`).
    /// - Components in [`CustomMix`] are sorted first by mole-fraction in descending order, then by
    ///   name in alphabetical order (for predictable results, since the
    ///   [`HashMap`](std::collections::HashMap) used to store [`CustomMix`] components does not
    ///   guarantee ordering).
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water: Substance = Pure::Water.into();
    /// assert_eq!(water.composition_id(), "Water");
    ///
    /// let incomp_water: Substance = IncompPure::Water.into();
    /// assert_eq!(incomp_water.composition_id(), "Water");
    ///
    /// let r444a: Substance = PredefinedMix::R444A.into();
    /// assert_eq!(r444a.composition_id(), "R444A.mix");
    ///
    /// let propylene_glycol: Substance = BinaryMixKind::MPG.with_fraction(0.4)?.into();
    /// assert_eq!(propylene_glycol.composition_id(), "MPG");
    ///
    /// let custom_mix: Substance =
    ///     CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)])?.into();
    ///
    /// // Components are sorted first by mole-fraction in descending order,
    /// // then by name in alphabetical order
    /// assert_eq!(custom_mix.composition_id(), "Water&Ethanol");
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    #[must_use]
    pub fn composition_id(&self) -> Cow<'static, str> {
        match self {
            Substance::BinaryMix(binary_mix) => Cow::Borrowed(binary_mix.kind.into()),
            Substance::CustomMix(custom_mix) => {
                let mix = custom_mix.clone().into_mole_based();
                let mut components: Vec<_> = mix.components().iter().collect();
                components.sort_by(|left, right| {
                    right.1.total_cmp(left.1).then_with(|| left.0.as_ref().cmp(right.0.as_ref()))
                });
                let name = components
                    .into_iter()
                    .map(|component| component.0.as_ref())
                    .collect::<Vec<_>>()
                    .join("&");
                Cow::Owned(name)
            }
            _ => self.name(),
        }
    }

    /// Aliases.
    ///
    /// # Notes
    ///
    /// Curiously, for [`CustomMix`], `CoolProp` reflects the first component only.
    #[must_use]
    pub fn aliases(&self) -> Vec<String> {
        let sep = config::read().list_punctuation;
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::Aliases)
            .map(|aliases| aliases.split(sep).map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Name used in `REFPROP`.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn refprop_name(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::RefpropName)
    }

    /// Chemical Abstracts Service (CAS) registry number.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn cas(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::Cas)
    }

    /// International Chemical Identifier (InChI).
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[allow(clippy::doc_markdown)]
    #[must_use]
    pub fn inchi(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::Inchi)
    }

    /// Hashed version of the International Chemical Identifier (InChIKey).
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[allow(clippy::doc_markdown)]
    #[must_use]
    pub fn inchi_key(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::InchiKey)
    }

    /// [`ChemSpider`](https://www.chemspider.com/) identifier.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn chemspider_id(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::ChemSpiderId)
    }

    /// Simplified Molecular Input Line Entry System (SMILES) string.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn smiles(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::Smiles)
    }

    /// ASHRAE Standard 34 safety rating.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn ashrae34(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::Ashrae34)
    }

    /// URL to a `2D` molecular structure image.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn two_d_png_url(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::TwoDPngUrl)
    }

    /// Equation of state BibTeX key.
    ///
    /// Returns [`None`] if not available for this substance.
    ///
    /// # Notes
    ///
    /// Curiously, for [`PredefinedMix`] and [`CustomMix`], `CoolProp` reflects
    /// the first component only.
    #[must_use]
    pub fn bibtex_eos(&self) -> Option<String> {
        CoolProp::get_substance_param(self.composition_id(), SubstanceParam::BibtexEos)
    }
}

impl From<Pure> for Substance {
    fn from(value: Pure) -> Self {
        Self::Pure(value)
    }
}

impl From<IncompPure> for Substance {
    fn from(value: IncompPure) -> Self {
        Self::IncompPure(value)
    }
}

impl From<PredefinedMix> for Substance {
    fn from(value: PredefinedMix) -> Self {
        Self::PredefinedMix(value)
    }
}

impl From<BinaryMix> for Substance {
    fn from(value: BinaryMix) -> Self {
        Self::BinaryMix(value)
    }
}

impl From<CustomMix> for Substance {
    fn from(value: CustomMix) -> Self {
        Self::CustomMix(value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::fluid::backend::BaseBackend;

    #[test]
    fn with_backend() {
        // Given
        let sut: Substance = Pure::Water.into();

        // When
        let res = sut.with_backend(BaseBackend::If97);

        // Then
        assert_eq!(res, SubstanceWithBackend { substance: sut, backend: BaseBackend::If97.into() });
    }

    #[test]
    fn with_default_backend() {
        // Given
        let sut: Substance = Pure::Water.into();

        // When
        let res = sut.with_default_backend();

        // Then
        assert_eq!(res, SubstanceWithBackend { substance: sut, backend: BaseBackend::Heos.into() });
    }

    #[test]
    fn into_with_backend() {
        // Given
        let sut: Substance = Pure::Water.into();

        // When
        let res = sut.into_with_backend(BaseBackend::If97);

        // Then
        assert_eq!(
            res,
            SubstanceWithBackend {
                substance: Pure::Water.into(),
                backend: BaseBackend::If97.into()
            }
        );
    }

    #[test]
    fn into_with_default_backend() {
        // Given
        let sut: Substance = Pure::Water.into();

        // When
        let res = sut.into_with_default_backend();

        // Then
        assert_eq!(
            res,
            SubstanceWithBackend {
                substance: Pure::Water.into(),
                backend: BaseBackend::Heos.into()
            }
        );
    }

    #[rstest]
    #[case(Pure::Water, "Water")]
    #[case(IncompPure::Water, "Water")]
    #[case(PredefinedMix::R444A, "R444A.mix")]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), "MPG[0.4]")]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        "Water[0.8]&Ethanol[0.2]"
    )]
    #[case(
        CustomMix::mole_based(
            [(Pure::Methanol, 0.1), (Pure::Ethanol, 0.1), (Pure::Water, 0.8)]
        ).unwrap(),
        "Water[0.8]&Ethanol[0.1]&Methanol[0.1]"
    )]
    fn name(#[case] sut: impl Into<Substance>, #[case] expected: &str) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.name();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(Pure::Water, "Water")]
    #[case(IncompPure::Water, "Water")]
    #[case(PredefinedMix::R444A, "R444A.mix")]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), "MPG")]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        "Water&Ethanol"
    )]
    #[case(
        CustomMix::mole_based(
            [(Pure::Methanol, 0.1), (Pure::Ethanol, 0.1), (Pure::Water, 0.8)]
        ).unwrap(),
        "Water&Ethanol&Methanol"
    )]
    fn composition_id(#[case] sut: impl Into<Substance>, #[case] expected: &str) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.composition_id();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(Pure::Water, vec!["water", "WATER", "H2O", "h2o", "R718"])]
    #[case(IncompPure::Water,  vec!["water", "WATER", "H2O", "h2o", "R718"])]
    #[case(PredefinedMix::R444A, Vec::new())]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), Vec::new())]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        vec!["water", "WATER", "H2O", "h2o", "R718"]
    )]
    fn aliases(#[case] sut: impl Into<Substance>, #[case] expected: Vec<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.aliases();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("WATER"))]
    #[case(IncompPure::Water, Some("WATER"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("R32"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("WATER")
    )]
    fn refprop_name(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.refprop_name();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("7732-18-5"))]
    #[case(IncompPure::Water, Some("7732-18-5"))]
    #[case(Pure::R32, Some("75-10-5"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("75-10-5"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("7732-18-5")
    )]
    fn cas(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.cas();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("InChI=1S/H2O/h1H2"))]
    #[case(IncompPure::Water, Some("InChI=1S/H2O/h1H2"))]
    #[case(Pure::R32, Some("InChI=1S/CH2F2/c2-1-3/h1H2"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("InChI=1S/CH2F2/c2-1-3/h1H2"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("InChI=1S/H2O/h1H2")
    )]
    fn inchi(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.inchi();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("XLYOFNOQVPJJNP-UHFFFAOYSA-N"))] // cspell: disable-line
    #[case(IncompPure::Water, Some("XLYOFNOQVPJJNP-UHFFFAOYSA-N"))] // cspell: disable-line
    #[case(Pure::R32, Some("RWRIWBAIICGTTQ-UHFFFAOYSA-N"))] // cspell: disable-line
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("RWRIWBAIICGTTQ-UHFFFAOYSA-N"))] // cspell: disable-line
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("XLYOFNOQVPJJNP-UHFFFAOYSA-N") // cspell: disable-line
    )]
    fn inchi_key(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.inchi_key();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("937"))]
    #[case(IncompPure::Water, Some("937"))]
    #[case(Pure::R32, Some("6105"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("6105"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("937")
    )]
    fn chemspider_id(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.chemspider_id();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("O"))]
    #[case(IncompPure::Water, Some("O"))]
    #[case(Pure::R32, Some("C(F)F"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("C(F)F"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("O")
    )]
    fn smiles(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.smiles();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("A1"))]
    #[case(IncompPure::Water, Some("A1"))]
    #[case(Pure::R32, Some("A2"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("A2"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("A1")
    )]
    fn ashrae34(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.ashrae34();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("http://www.chemspider.com/ImagesHandler.ashx?id=937"))]
    #[case(IncompPure::Water, Some("http://www.chemspider.com/ImagesHandler.ashx?id=937"))]
    #[case(Pure::R32, Some("http://www.chemspider.com/ImagesHandler.ashx?id=6105"))]
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("http://www.chemspider.com/ImagesHandler.ashx?id=6105"))]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("http://www.chemspider.com/ImagesHandler.ashx?id=937")
    )]
    fn two_d_png_url(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.two_d_png_url();

        // Then
        assert_eq!(res.as_deref(), expected);
    }

    #[rstest]
    #[case(Pure::Water, Some("Wagner-JPCRD-2002"))] // cspell: disable-line
    #[case(IncompPure::Water, Some("Wagner-JPCRD-2002"))] // cspell: disable-line
    #[case(Pure::R32, Some("TillnerRoth-JPCRD-1997"))] // cspell: disable-line
    // CoolProp reflects the first component only
    #[case(PredefinedMix::R444A, Some("TillnerRoth-JPCRD-1997"))] // cspell: disable-line
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), None)]
    #[case(
        CustomMix::mole_based([(Pure::Ethanol, 0.2), (Pure::Water, 0.8)]).unwrap(),
        // CoolProp reflects the first component only
        Some("Wagner-JPCRD-2002") // cspell: disable-line
    )]
    fn bibtex_eos(#[case] sut: impl Into<Substance>, #[case] expected: Option<&str>) {
        // Given
        let sut: Substance = sut.into();

        // When
        let res = sut.bibtex_eos();

        // Then
        assert_eq!(res.as_deref(), expected);
    }
}
