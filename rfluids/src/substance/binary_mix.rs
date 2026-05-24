// cSpell:disable

use std::str::FromStr;

use strum::EnumProperty;

/// `CoolProp` incompressible binary mixtures _(mass-based or volume-based)_.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
///
/// use rfluids::prelude::*;
///
/// assert_eq!(BinaryMixKind::MPG.as_ref(), "MPG");
/// assert_eq!(BinaryMixKind::from_str("MPG"), Ok(BinaryMixKind::MPG));
/// assert_eq!(BinaryMixKind::try_from("MPG"), Ok(BinaryMixKind::MPG));
/// ```
///
/// # See Also
///
/// - [Incompressible Substances](https://coolprop.org/fluid_properties/Incompressibles.html)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumProperty,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum BinaryMixKind {
    /// Freezium potassium formate solution.
    ///
    /// # See Also
    ///
    /// - [FRE Fitting Report](https://coolprop.org/_downloads/34957f16ba277ebf93b5b6c77ba773ef/FRE_fitreport.pdf)
    #[strum(to_string = "FRE", props(min_fraction = "0.19", max_fraction = "0.5"))]
    FRE,

    /// Ice slurry with ethanol.
    ///
    /// # See Also
    ///
    /// - [IceEA Fitting Report](https://coolprop.org/_downloads/e4168abbd58ed6ddde675dd652d4ad03/IceEA_fitreport.pdf)
    #[strum(to_string = "IceEA", props(min_fraction = "0.05", max_fraction = "0.35"))]
    IceEA,

    /// Ice slurry with sodium chloride.
    ///
    /// # See Also
    ///
    /// - [IceNA Fitting Report](https://coolprop.org/_downloads/59f395394a1345b1da409186610afdf6/IceNA_fitreport.pdf)
    #[strum(to_string = "IceNA", props(min_fraction = "0.05", max_fraction = "0.35"))]
    IceNA,

    /// Ice slurry with propylene glycol.
    ///
    /// # See Also
    ///
    /// - [IcePG Fitting Report](https://coolprop.org/_downloads/f9efc1eeca84e7435e3d0552869a3e72/IcePG_fitreport.pdf)
    #[strum(to_string = "IcePG", props(min_fraction = "0.05", max_fraction = "0.35"))]
    IcePG,

    /// Lithium bromide aqueous solution.
    ///
    /// # See Also
    ///
    /// - [LiBr Fitting Report](https://coolprop.org/_downloads/9841afe5a67cf511e1c05fb2c62554e9/LiBr_fitreport.pdf)
    #[strum(to_string = "LiBr", props(min_fraction = "0.0", max_fraction = "0.75"))]
    LiBr,

    /// Ammonia aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MAM Fitting Report](https://coolprop.org/_downloads/55f5945b82e59783f01a223c806ba98a/MAM_fitreport.pdf)
    #[strum(to_string = "MAM", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MAM,

    /// Melinder ammonia aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MAM2 Fitting Report](https://coolprop.org/_downloads/80ceb7a3f0ef19024df7102ede7abf1c/MAM2_fitreport.pdf)
    #[strum(to_string = "MAM2", props(min_fraction = "0.078", max_fraction = "0.236"))]
    MAM2,

    /// Calcium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MCA Fitting Report](https://coolprop.org/_downloads/99c24e500b86d54a8ee06ec42fe88650/MCA_fitreport.pdf)
    #[strum(to_string = "MCA", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MCA,

    /// Melinder calcium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MCA2 Fitting Report](https://coolprop.org/_downloads/7f1f22344947f75279e127d261ed1bb4/MCA2_fitreport.pdf)
    #[strum(to_string = "MCA2", props(min_fraction = "0.09", max_fraction = "0.294"))]
    MCA2,

    /// Ethanol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MEA Fitting Report](https://coolprop.org/_downloads/9766a2a46a5ad683a2421fba9a8bbb00/MEA_fitreport.pdf)
    #[strum(to_string = "MEA", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MEA,

    /// Melinder ethanol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MEA2 Fitting Report](https://coolprop.org/_downloads/8702b193f54b78d69b863cd9a751554b/MEA2_fitreport.pdf)
    #[strum(to_string = "MEA2", props(min_fraction = "0.11", max_fraction = "0.6"))]
    MEA2,

    /// Ethylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MEG Fitting Report](https://coolprop.org/_downloads/4e6bd34ef1888b4daa1662944ee0ea7e/MEG_fitreport.pdf)
    #[strum(to_string = "MEG", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MEG,

    /// Melinder ethylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MEG2 Fitting Report](https://coolprop.org/_downloads/eb2feab3610b6184c36a49d7555a9a8b/MEG2_fitreport.pdf)
    #[strum(to_string = "MEG2", props(min_fraction = "0.0", max_fraction = "0.56"))]
    MEG2,

    /// Glycerol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MGL Fitting Report](https://coolprop.org/_downloads/3f422f64ed4d455fc1cf4ea9d008d489/MGL_fitreport.pdf)
    #[strum(to_string = "MGL", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MGL,

    /// Melinder glycerol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MGL2 Fitting Report](https://coolprop.org/_downloads/9129c94c0b61a92d578721eb3244a8d9/MGL2_fitreport.pdf)
    #[strum(to_string = "MGL2", props(min_fraction = "0.195", max_fraction = "0.63"))]
    MGL2,

    /// MIT seawater model.
    ///
    /// # See Also
    ///
    /// - [MITSW Fitting Report](https://coolprop.org/_downloads/83b323e2117a318d6a908a62332755a0/MITSW_fitreport.pdf)
    #[strum(to_string = "MITSW", props(min_fraction = "0.0", max_fraction = "0.12"))]
    MITSW,

    /// Potassium acetate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MKA Fitting Report](https://coolprop.org/_downloads/25c70020117c037c1bda6b82064cb6a7/MKA_fitreport.pdf)
    #[strum(to_string = "MKA", props(min_fraction = "0.0", max_fraction = "0.45"))]
    MKA,

    /// Melinder potassium acetate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MKA2 Fitting Report](https://coolprop.org/_downloads/eb05fcec89ff48ef20aa1f42e2f3fcaf/MKA2_fitreport.pdf)
    #[strum(to_string = "MKA2", props(min_fraction = "0.11", max_fraction = "0.41"))]
    MKA2,

    /// Potassium carbonate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MKC Fitting Report](https://coolprop.org/_downloads/af83599a6bdebd138597d0544fe81250/MKC_fitreport.pdf)
    #[strum(to_string = "MKC", props(min_fraction = "0.0", max_fraction = "0.4"))]
    MKC,

    /// Melinder potassium carbonate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MKC2 Fitting Report](https://coolprop.org/_downloads/af88aff662b6e97de038fe5ab1099d56/MKC2_fitreport.pdf)
    #[strum(to_string = "MKC2", props(min_fraction = "0.0", max_fraction = "0.39"))]
    MKC2,

    /// Potassium formate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MKF Fitting Report](https://coolprop.org/_downloads/0a1ac8e2d2b9f530ff25443a274a63a1/MKF_fitreport.pdf)
    #[strum(to_string = "MKF", props(min_fraction = "0.0", max_fraction = "0.48"))]
    MKF,

    /// Lithium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MLI Fitting Report](https://coolprop.org/_downloads/52820bc9842eaa443aae3ff6fbad9c6f/MLI_fitreport.pdf)
    #[strum(to_string = "MLI", props(min_fraction = "0.0", max_fraction = "0.24"))]
    MLI,

    /// Methanol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MMA Fitting Report](https://coolprop.org/_downloads/cd121df9ebf7251a0e4d20ded28c4a7f/MMA_fitreport.pdf)
    #[strum(to_string = "MMA", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MMA,

    /// Melinder methanol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MMA2 Fitting Report](https://coolprop.org/_downloads/ac5e575a586e468e1df740bba389068a/MMA2_fitreport.pdf)
    #[strum(to_string = "MMA2", props(min_fraction = "0.078", max_fraction = "0.474"))]
    MMA2,

    /// Magnesium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MMG Fitting Report](https://coolprop.org/_downloads/65b64f4f312d7711f888d48e78607307/MMG_fitreport.pdf)
    #[strum(to_string = "MMG", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MMG,

    /// Melinder magnesium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MMG2 Fitting Report](https://coolprop.org/_downloads/3f60e26eeef0be3d41f2bf5410c6456e/MMG2_fitreport.pdf)
    #[strum(to_string = "MMG2", props(min_fraction = "0.0", max_fraction = "0.205"))]
    MMG2,

    /// Sodium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MNA Fitting Report](https://coolprop.org/_downloads/70b1723cc03bf11236be1da5364ad2a3/MNA_fitreport.pdf)
    #[strum(to_string = "MNA", props(min_fraction = "0.0", max_fraction = "0.23"))]
    MNA,

    /// Melinder sodium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MNA2 Fitting Report](https://coolprop.org/_downloads/42a537083fe286eb02364dea3ee11688/MNA2_fitreport.pdf)
    #[strum(to_string = "MNA2", props(min_fraction = "0.0", max_fraction = "0.23"))]
    MNA2,

    /// Propylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MPG Fitting Report](https://coolprop.org/_downloads/15da2eff4264ab41969c2288a2fd7a14/MPG_fitreport.pdf)
    #[strum(to_string = "MPG", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MPG,

    /// Melinder propylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [MPG2 Fitting Report](https://coolprop.org/_downloads/51fc2618b7bf9df9eece1b789ed1dd76/MPG2_fitreport.pdf)
    #[strum(to_string = "MPG2", props(min_fraction = "0.15", max_fraction = "0.57"))]
    MPG2,

    /// VDI calcium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [VCA Fitting Report](https://coolprop.org/_downloads/3d545447ca28deec34e87f4e2c7372e5/VCA_fitreport.pdf)
    #[strum(to_string = "VCA", props(min_fraction = "0.147", max_fraction = "0.299"))]
    VCA,

    /// VDI potassium carbonate aqueous solution.
    ///
    /// # See Also
    ///
    /// - [VKC Fitting Report](https://coolprop.org/_downloads/f5f494464726eef64bef309a06a231b9/VKC_fitreport.pdf)
    #[strum(to_string = "VKC", props(min_fraction = "0.128", max_fraction = "0.389"))]
    VKC,

    /// VDI methanol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [VMA Fitting Report](https://coolprop.org/_downloads/8fbd3dad595acf18e51ebc9db74bf4db/VMA_fitreport.pdf)
    #[strum(to_string = "VMA", props(min_fraction = "0.1", max_fraction = "0.9"))]
    VMA,

    /// VDI magnesium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [VMG Fitting Report](https://coolprop.org/_downloads/85befc9ba3e20979e025731d8b27461a/VMG_fitreport.pdf)
    #[strum(to_string = "VMG", props(min_fraction = "0.072", max_fraction = "0.206"))]
    VMG,

    /// VDI sodium chloride aqueous solution.
    ///
    /// # See Also
    ///
    /// - [VNA Fitting Report](https://coolprop.org/_downloads/38a80b6ad28ae9ce214391a3c87c962e/VNA_fitreport.pdf)
    #[strum(to_string = "VNA", props(min_fraction = "0.07", max_fraction = "0.231"))]
    VNA,

    /// ASHRAE ethylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [AEG Fitting Report](https://coolprop.org/_downloads/78e13c7e1ac35bce0c6f3d8da3beb923/AEG_fitreport.pdf)
    #[strum(to_string = "AEG", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AEG,

    /// Antifrogen KF potassium formate solution.
    ///
    /// # See Also
    ///
    /// - [AKF Fitting Report](https://coolprop.org/_downloads/84382d0deabf6c4714e8bce7d785d47c/AKF_fitreport.pdf)
    #[strum(to_string = "AKF", props(min_fraction = "0.4", max_fraction = "1.0"))]
    AKF,

    /// Antifrogen L propylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [AL Fitting Report](https://coolprop.org/_downloads/aa4571c7edf7b7a96d3fd0880cd6ab7e/AL_fitreport.pdf)
    #[strum(to_string = "AL", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AL,

    /// Antifrogen N ethylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [AN Fitting Report](https://coolprop.org/_downloads/c77b6564ae1467f00cc06ee0cdfc96f0/AN_fitreport.pdf)
    #[strum(to_string = "AN", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AN,

    /// ASHRAE propylene glycol aqueous solution.
    ///
    /// # See Also
    ///
    /// - [APG Fitting Report](https://coolprop.org/_downloads/ff81838c5666d41cd5cf3b4cbbdbae99/APG_fitreport.pdf)
    #[strum(to_string = "APG", props(min_fraction = "0.1", max_fraction = "0.6"))]
    APG,

    /// Glykosol N ethylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [GKN Fitting Report](https://coolprop.org/_downloads/608e1caabc90a2a8decbe2c67e2820f5/GKN_fitreport.pdf)
    #[strum(to_string = "GKN", props(min_fraction = "0.1", max_fraction = "0.6"))]
    GKN,

    /// Pekasol 2000 potassium acetate/formate solution.
    ///
    /// # See Also
    ///
    /// - [PK2 Fitting Report](https://coolprop.org/_downloads/f8ea5374d239a80eb1f8bbc028d6e843/PK2_fitreport.pdf)
    #[strum(to_string = "PK2", props(min_fraction = "0.3", max_fraction = "1.0"))]
    PK2,

    /// Pekasol L propylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [PKL Fitting Report](https://coolprop.org/_downloads/6886474c8959679625eb06ab486a039e/PKL_fitreport.pdf)
    #[strum(to_string = "PKL", props(min_fraction = "0.1", max_fraction = "0.6"))]
    PKL,

    /// Zitrec AC corrosion inhibitor solution.
    ///
    /// # See Also
    ///
    /// - [ZAC Fitting Report](https://coolprop.org/_downloads/2eb5c385aa7a082fbf631d7c95cb200b/ZAC_fitreport.pdf)
    #[strum(to_string = "ZAC", props(min_fraction = "0.06", max_fraction = "0.5"))]
    ZAC,

    /// Zitrec FC propylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [ZFC Fitting Report](https://coolprop.org/_downloads/2161e01af1f1fa119bc0a8d285e889f4/ZFC_fitreport.pdf)
    #[strum(to_string = "ZFC", props(min_fraction = "0.3", max_fraction = "0.6"))]
    ZFC,

    /// Zitrec LC propylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [ZLC Fitting Report](https://coolprop.org/_downloads/56d3bcd6e7b9021ac6f75168e4dbfd86/ZLC_fitreport.pdf)
    #[strum(to_string = "ZLC", props(min_fraction = "0.3", max_fraction = "0.7"))]
    ZLC,

    /// Zitrec M ethylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [ZM Fitting Report](https://coolprop.org/_downloads/d56dacbaa99ddd7f467a8e07288d72cd/ZM_fitreport.pdf)
    #[strum(to_string = "ZM", props(min_fraction = "0.0", max_fraction = "1.0"))]
    ZM,

    /// Zitrec MC ethylene glycol solution.
    ///
    /// # See Also
    ///
    /// - [ZMC Fitting Report](https://coolprop.org/_downloads/31e9c35471e97108ec2590b310cfcbfe/ZMC_fitreport.pdf)
    #[strum(to_string = "ZMC", props(min_fraction = "0.3", max_fraction = "0.7"))]
    ZMC,
}

impl BinaryMixKind {
    /// Minimum possible fraction **\[dimensionless, from 0 to 1\]**.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// assert_eq!(BinaryMixKind::MPG.min_fraction(), 0.0);
    /// ```
    #[must_use]
    pub fn min_fraction(&self) -> f64 {
        f64::from_str(self.get_str("min_fraction").unwrap()).unwrap()
    }

    /// Maximum possible fraction **\[dimensionless, from 0 to 1\]**.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// assert_eq!(BinaryMixKind::MPG.max_fraction(), 0.6);
    /// ```
    #[must_use]
    pub fn max_fraction(&self) -> f64 {
        f64::from_str(self.get_str("max_fraction").unwrap()).unwrap()
    }

    /// Creates and returns a new [`BinaryMix`] instance.
    ///
    /// # Arguments
    ///
    /// - `fraction` -- specified fraction **\[dimensionless, from 0 to 1\]**
    ///
    /// # Errors
    ///
    /// Returns a [`BinaryMixError`] if the fraction is out of the valid range
    /// [[`min_fraction`](BinaryMixKind::min_fraction);
    /// [`max_fraction`](BinaryMixKind::max_fraction)].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// assert!(BinaryMixKind::MPG.with_fraction(0.4).is_ok());
    /// assert!(BinaryMixKind::MPG.with_fraction(1.0).is_err());
    /// ```
    pub fn with_fraction(self, fraction: f64) -> Result<BinaryMix, BinaryMixError> {
        if !(self.min_fraction()..=self.max_fraction()).contains(&fraction) {
            return Err(BinaryMixError::InvalidFraction {
                specified: fraction,
                min: self.min_fraction(),
                max: self.max_fraction(),
            });
        }
        Ok(BinaryMix { kind: self, fraction })
    }
}

/// [`BinaryMixKind`] with specified fraction _(mass-based or volume-based)_.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct BinaryMix {
    /// Specified kind.
    pub kind: BinaryMixKind,
    /// Specified fraction **\[dimensionless, from 0 to 1\]**.
    pub fraction: f64,
}

/// Error during creation of [`BinaryMix`].
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BinaryMixError {
    /// Specified fraction is invalid.
    #[error("specified fraction `{specified:?}` is out of possible range [{min:.1}; {max:.1}]")]
    InvalidFraction {
        /// Specified value.
        specified: f64,
        /// Minimum possible value.
        min: f64,
        /// Maximum possible value.
        max: f64,
    },
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use strum::IntoEnumIterator;

    use super::{BinaryMixKind::*, *};

    #[rstest]
    #[case(FRE, 0.19, 0.5)]
    #[case(IceEA, 0.05, 0.35)]
    #[case(IceNA, 0.05, 0.35)]
    #[case(IcePG, 0.05, 0.35)]
    #[case(LiBr, 0.0, 0.75)]
    #[case(MAM, 0.0, 0.3)]
    #[case(MAM2, 0.078, 0.236)]
    #[case(MCA, 0.0, 0.3)]
    #[case(MCA2, 0.09, 0.294)]
    #[case(MEA, 0.0, 0.6)]
    #[case(MEA2, 0.11, 0.6)]
    #[case(MEG, 0.0, 0.6)]
    #[case(MEG2, 0.0, 0.56)]
    #[case(MGL, 0.0, 0.6)]
    #[case(MGL2, 0.195, 0.63)]
    #[case(MITSW, 0.0, 0.12)]
    #[case(MKA, 0.0, 0.45)]
    #[case(MKA2, 0.11, 0.41)]
    #[case(MKC, 0.0, 0.4)]
    #[case(MKC2, 0.0, 0.39)]
    #[case(MKF, 0.0, 0.48)]
    #[case(MLI, 0.0, 0.24)]
    #[case(MMA, 0.0, 0.6)]
    #[case(MMA2, 0.078, 0.474)]
    #[case(MMG, 0.0, 0.3)]
    #[case(MMG2, 0.0, 0.205)]
    #[case(MNA, 0.0, 0.23)]
    #[case(MNA2, 0.0, 0.23)]
    #[case(MPG, 0.0, 0.6)]
    #[case(MPG2, 0.15, 0.57)]
    #[case(VCA, 0.147, 0.299)]
    #[case(VKC, 0.128, 0.389)]
    #[case(VMA, 0.1, 0.9)]
    #[case(VMG, 0.072, 0.206)]
    #[case(VNA, 0.07, 0.231)]
    #[case(AEG, 0.1, 0.6)]
    #[case(AKF, 0.4, 1.0)]
    #[case(AL, 0.1, 0.6)]
    #[case(AN, 0.1, 0.6)]
    #[case(APG, 0.1, 0.6)]
    #[case(GKN, 0.1, 0.6)]
    #[case(PK2, 0.3, 1.0)]
    #[case(PKL, 0.1, 0.6)]
    #[case(ZAC, 0.06, 0.5)]
    #[case(ZFC, 0.3, 0.6)]
    #[case(ZLC, 0.3, 0.7)]
    #[case(ZM, 0.0, 1.0)]
    #[case(ZMC, 0.3, 0.7)]
    fn min_max_fractions(
        #[case] sut: BinaryMixKind,
        #[case] expected_min: f64,
        #[case] expected_max: f64,
    ) {
        // When
        let (min, max) = (sut.min_fraction(), sut.max_fraction());

        // Then
        assert_eq!(min, expected_min);
        assert_eq!(max, expected_max);
    }

    #[test]
    fn with_fraction_valid() {
        for kind in BinaryMixKind::iter() {
            // Given
            let min = kind.min_fraction();
            let average = 0.5 * (kind.min_fraction() + kind.max_fraction());
            let max = kind.max_fraction();

            // When
            let BinaryMix { kind: kind1, fraction: fraction1 } = kind.with_fraction(min).unwrap();
            let BinaryMix { kind: kind2, fraction: fraction2 } =
                kind.with_fraction(average).unwrap();
            let BinaryMix { kind: kind3, fraction: fraction3 } = kind.with_fraction(max).unwrap();

            // Then
            assert_eq!(kind1, kind);
            assert_eq!(kind2, kind);
            assert_eq!(kind3, kind);
            assert_eq!(fraction1, min);
            assert_eq!(fraction2, average);
            assert_eq!(fraction3, max);
        }
    }

    #[test]
    fn with_fraction_invalid() {
        const DELTA: f64 = 1e-6;
        for kind in BinaryMixKind::iter() {
            // Given
            let too_low = kind.min_fraction() - DELTA;
            let too_high = kind.max_fraction() + DELTA;

            // When
            let res1 = kind.with_fraction(too_low);
            let res2 = kind.with_fraction(too_high);

            // Then
            assert_eq!(
                res1,
                Err(BinaryMixError::InvalidFraction {
                    specified: too_low,
                    min: kind.min_fraction(),
                    max: kind.max_fraction(),
                })
            );
            assert_eq!(
                res2,
                Err(BinaryMixError::InvalidFraction {
                    specified: too_high,
                    min: kind.min_fraction(),
                    max: kind.max_fraction(),
                })
            );
        }
    }

    #[rstest]
    #[case(FRE, "FRE")]
    #[case(IceEA, "IceEA")]
    #[case(IceNA, "IceNA")]
    #[case(IcePG, "IcePG")]
    #[case(LiBr, "LiBr")]
    #[case(MAM, "MAM")]
    #[case(MAM2, "MAM2")]
    #[case(MCA, "MCA")]
    #[case(MCA2, "MCA2")]
    #[case(MEA, "MEA")]
    #[case(MEA2, "MEA2")]
    #[case(MEG, "MEG")]
    #[case(MEG2, "MEG2")]
    #[case(MGL, "MGL")]
    #[case(MGL2, "MGL2")]
    #[case(MITSW, "MITSW")]
    #[case(MKA, "MKA")]
    #[case(MKA2, "MKA2")]
    #[case(MKC, "MKC")]
    #[case(MKC2, "MKC2")]
    #[case(MKF, "MKF")]
    #[case(MLI, "MLI")]
    #[case(MMA, "MMA")]
    #[case(MMA2, "MMA2")]
    #[case(MMG, "MMG")]
    #[case(MMG2, "MMG2")]
    #[case(MNA, "MNA")]
    #[case(MNA2, "MNA2")]
    #[case(MPG, "MPG")]
    #[case(MPG2, "MPG2")]
    #[case(VCA, "VCA")]
    #[case(VKC, "VKC")]
    #[case(VMA, "VMA")]
    #[case(VMG, "VMG")]
    #[case(VNA, "VNA")]
    #[case(AEG, "AEG")]
    #[case(AKF, "AKF")]
    #[case(AL, "AL")]
    #[case(AN, "AN")]
    #[case(APG, "APG")]
    #[case(GKN, "GKN")]
    #[case(PK2, "PK2")]
    #[case(PKL, "PKL")]
    #[case(ZAC, "ZAC")]
    #[case(ZFC, "ZFC")]
    #[case(ZLC, "ZLC")]
    #[case(ZM, "ZM")]
    #[case(ZMC, "ZMC")]
    fn as_str(#[case] sut: BinaryMixKind, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case("FRE", FRE)]
    #[case("IceEA", IceEA)]
    #[case("IceNA", IceNA)]
    #[case("IcePG", IcePG)]
    #[case("LiBr", LiBr)]
    #[case("MAM", MAM)]
    #[case("MAM2", MAM2)]
    #[case("MCA", MCA)]
    #[case("MCA2", MCA2)]
    #[case("MEA", MEA)]
    #[case("MEA2", MEA2)]
    #[case("MEG", MEG)]
    #[case("MEG2", MEG2)]
    #[case("MGL", MGL)]
    #[case("MGL2", MGL2)]
    #[case("MITSW", MITSW)]
    #[case("MKA", MKA)]
    #[case("MKA2", MKA2)]
    #[case("MKC", MKC)]
    #[case("MKC2", MKC2)]
    #[case("MKF", MKF)]
    #[case("MLI", MLI)]
    #[case("MMA", MMA)]
    #[case("MMA2", MMA2)]
    #[case("MMG", MMG)]
    #[case("MMG2", MMG2)]
    #[case("MNA", MNA)]
    #[case("MNA2", MNA2)]
    #[case("MPG", MPG)]
    #[case("MPG2", MPG2)]
    #[case("VCA", VCA)]
    #[case("VKC", VKC)]
    #[case("VMA", VMA)]
    #[case("VMG", VMG)]
    #[case("VNA", VNA)]
    #[case("AEG", AEG)]
    #[case("AKF", AKF)]
    #[case("AL", AL)]
    #[case("AN", AN)]
    #[case("APG", APG)]
    #[case("GKN", GKN)]
    #[case("PK2", PK2)]
    #[case("PKL", PKL)]
    #[case("ZAC", ZAC)]
    #[case("ZFC", ZFC)]
    #[case("ZLC", ZLC)]
    #[case("ZM", ZM)]
    #[case("ZMC", ZMC)]
    fn from_valid_str(#[case] valid: &str, #[case] expected: BinaryMixKind) {
        // When
        let res1 = BinaryMixKind::from_str(valid).unwrap();
        let res2 = BinaryMixKind::try_from(valid).unwrap();

        // Then
        assert_eq!(res1, expected);
        assert_eq!(res2, expected);
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str(#[case] invalid: &str) {
        // When
        let res1 = BinaryMixKind::from_str(invalid);
        let res2 = BinaryMixKind::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
