use super::FluidParam;

/// `CoolProp` fluids input pairs.
///
/// # Examples
///
/// Conversion to [`u8`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(u8::from(FluidInputPair::PT), 17);
/// ```
///
/// Conversion between two [`FluidParam`]s:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(
///     <(FluidParam, FluidParam)>::from(FluidInputPair::PT),
///     (FluidParam::P, FluidParam::T)
/// );
/// assert_eq!(FluidInputPair::try_from((FluidParam::T, FluidParam::P)), Ok(FluidInputPair::PT));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FluidInputPair {
    /// Mole-based vapor quality **\[dimensionless, from 0 to 1\]**, temperature **\[K\]**.
    QT = 1,

    /// Mass-based vapor quality **\[dimensionless, from 0 to 1\]**, temperature **\[K\]**.
    QMassT = 2,

    /// Pressure **\[Pa\]**, mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    PQ = 3,

    /// Pressure **\[Pa\]**, mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    PQMass = 4,

    /// Mole-based vapor quality **\[dimensionless, from 0 to 1\]**,
    /// molar specific entropy **\[J/mol/K\]**.
    QSMolar = 5,

    /// Mass-based vapor quality **\[dimensionless, from 0 to 1\]**,
    /// molar specific entropy **\[J/mol/K\]**.
    QMassSMolar = 6,

    /// Mole-based vapor quality **\[dimensionless, from 0 to 1\]**,
    /// mass specific entropy **\[J/kg/K\]**.
    QSMass = 7,

    /// Mass-based vapor quality **\[dimensionless, from 0 to 1\]**,
    /// mass specific entropy **\[J/kg/K\]**.
    QMassSMass = 8,

    /// Molar specific enthalpy **\[J/mol\]**,
    /// mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    HMolarQ = 9,

    /// Molar specific enthalpy **\[J/mol\]**,
    /// mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    HMolarQMass = 10,

    /// Mass specific enthalpy **\[J/kg\]**,
    /// mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    HMassQ = 11,

    /// Mass specific enthalpy **\[J/kg\]**,
    /// mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    HMassQMass = 12,

    /// Molar density **\[mol/m³\]**, mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    DMolarQ = 13,

    /// Molar density **\[mol/m³\]**, mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    DMolarQMass = 14,

    /// Mass density **\[kg/m³\]**, mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    DMassQ = 15,

    /// Mass density **\[kg/m³\]**, mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    DMassQMass = 16,

    /// Pressure **\[Pa\]**, temperature **\[K\]**.
    PT = 17,

    /// Mass density **\[kg/m³\]**, temperature **\[K\]**.
    DMassT = 18,

    /// Molar density **\[mol/m³\]**, temperature **\[K\]**.
    DMolarT = 19,

    /// Molar specific enthalpy **\[J/mol\]**, temperature **\[K\]**.
    HMolarT = 20,

    /// Mass specific enthalpy **\[J/kg\]**, temperature **\[K\]**.
    HMassT = 21,

    /// Molar specific entropy **\[J/mol/K\]**, temperature **\[K\]**.
    SMolarT = 22,

    /// Mass specific entropy **\[J/kg/K\]**, temperature **\[K\]**.
    SMassT = 23,

    /// Temperature **\[K\]**, molar specific internal energy **\[J/mol\]**.
    TUMolar = 24,

    /// Temperature **\[K\]**, mass specific internal energy **\[J/kg\]**.
    TUMass = 25,

    /// Mass density **\[kg/m³\]**, pressure **\[Pa\]**.
    DMassP = 26,

    /// Molar density **\[mol/m³\]**, pressure **\[Pa\]**.
    DMolarP = 27,

    /// Mass specific enthalpy **\[J/kg\]**, pressure **\[Pa\]**.
    HMassP = 28,

    /// Molar specific enthalpy **\[J/mol\]**, pressure **\[Pa\]**.
    HMolarP = 29,

    /// Pressure **\[Pa\]**, mass specific entropy **\[J/kg/K\]**.
    PSMass = 30,

    /// Pressure **\[Pa\]**, molar specific entropy **\[J/mol/K\]**.
    PSMolar = 31,

    /// Pressure **\[Pa\]**, mass specific internal energy **\[J/kg\]**.
    PUMass = 32,

    /// Pressure **\[Pa\]**, molar specific internal energy **\[J/mol\]**.
    PUMolar = 33,

    /// Mass specific enthalpy **\[J/kg\]**, mass specific entropy **\[J/kg/K\]**.
    HMassSMass = 34,

    /// Molar specific enthalpy **\[J/mol\]**, molar specific entropy **\[J/mol/K\]**.
    HMolarSMolar = 35,

    /// Mass specific entropy **\[J/kg/K\]**, mass specific internal energy **\[J/kg\]**.
    SMassUMass = 36,

    /// Molar specific entropy **\[J/mol/K\]**, molar specific internal energy **\[J/mol\]**.
    SMolarUMolar = 37,

    /// Mass density **\[kg/m³\]**, mass specific enthalpy **\[J/kg\]**.
    DMassHMass = 38,

    /// Molar density **\[mol/m³\]**, molar specific enthalpy **\[J/mol\]**.
    DMolarHMolar = 39,

    /// Mass density **\[kg/m³\]**, mass specific entropy **\[J/kg/K\]**.
    DMassSMass = 40,

    /// Molar density **\[mol/m³\]**, molar specific entropy **\[J/mol/K\]**.
    DMolarSMolar = 41,

    /// Mass density **\[kg/m³\]**, mass specific internal energy **\[J/kg\]**.
    DMassUMass = 42,

    /// Molar density **\[mol/m³\]**, molar specific internal energy **\[J/mol\]**.
    DMolarUMolar = 43,
}

impl From<FluidInputPair> for u8 {
    fn from(value: FluidInputPair) -> Self {
        value as u8
    }
}

impl From<FluidInputPair> for (FluidParam, FluidParam) {
    fn from(value: FluidInputPair) -> Self {
        match value {
            FluidInputPair::QT => (FluidParam::Q, FluidParam::T),
            FluidInputPair::QMassT => (FluidParam::QMass, FluidParam::T),
            FluidInputPair::PQ => (FluidParam::P, FluidParam::Q),
            FluidInputPair::PQMass => (FluidParam::P, FluidParam::QMass),
            FluidInputPair::QSMolar => (FluidParam::Q, FluidParam::SMolar),
            FluidInputPair::QMassSMolar => (FluidParam::QMass, FluidParam::SMolar),
            FluidInputPair::QSMass => (FluidParam::Q, FluidParam::SMass),
            FluidInputPair::QMassSMass => (FluidParam::QMass, FluidParam::SMass),
            FluidInputPair::HMolarQ => (FluidParam::HMolar, FluidParam::Q),
            FluidInputPair::HMolarQMass => (FluidParam::HMolar, FluidParam::QMass),
            FluidInputPair::HMassQ => (FluidParam::HMass, FluidParam::Q),
            FluidInputPair::HMassQMass => (FluidParam::HMass, FluidParam::QMass),
            FluidInputPair::DMolarQ => (FluidParam::DMolar, FluidParam::Q),
            FluidInputPair::DMolarQMass => (FluidParam::DMolar, FluidParam::QMass),
            FluidInputPair::DMassQ => (FluidParam::DMass, FluidParam::Q),
            FluidInputPair::DMassQMass => (FluidParam::DMass, FluidParam::QMass),
            FluidInputPair::PT => (FluidParam::P, FluidParam::T),
            FluidInputPair::DMassT => (FluidParam::DMass, FluidParam::T),
            FluidInputPair::DMolarT => (FluidParam::DMolar, FluidParam::T),
            FluidInputPair::HMolarT => (FluidParam::HMolar, FluidParam::T),
            FluidInputPair::HMassT => (FluidParam::HMass, FluidParam::T),
            FluidInputPair::SMolarT => (FluidParam::SMolar, FluidParam::T),
            FluidInputPair::SMassT => (FluidParam::SMass, FluidParam::T),
            FluidInputPair::TUMolar => (FluidParam::T, FluidParam::UMolar),
            FluidInputPair::TUMass => (FluidParam::T, FluidParam::UMass),
            FluidInputPair::DMassP => (FluidParam::DMass, FluidParam::P),
            FluidInputPair::DMolarP => (FluidParam::DMolar, FluidParam::P),
            FluidInputPair::HMassP => (FluidParam::HMass, FluidParam::P),
            FluidInputPair::HMolarP => (FluidParam::HMolar, FluidParam::P),
            FluidInputPair::PSMass => (FluidParam::P, FluidParam::SMass),
            FluidInputPair::PSMolar => (FluidParam::P, FluidParam::SMolar),
            FluidInputPair::PUMass => (FluidParam::P, FluidParam::UMass),
            FluidInputPair::PUMolar => (FluidParam::P, FluidParam::UMolar),
            FluidInputPair::HMassSMass => (FluidParam::HMass, FluidParam::SMass),
            FluidInputPair::HMolarSMolar => (FluidParam::HMolar, FluidParam::SMolar),
            FluidInputPair::SMassUMass => (FluidParam::SMass, FluidParam::UMass),
            FluidInputPair::SMolarUMolar => (FluidParam::SMolar, FluidParam::UMolar),
            FluidInputPair::DMassHMass => (FluidParam::DMass, FluidParam::HMass),
            FluidInputPair::DMolarHMolar => (FluidParam::DMolar, FluidParam::HMolar),
            FluidInputPair::DMassSMass => (FluidParam::DMass, FluidParam::SMass),
            FluidInputPair::DMolarSMolar => (FluidParam::DMolar, FluidParam::SMolar),
            FluidInputPair::DMassUMass => (FluidParam::DMass, FluidParam::UMass),
            FluidInputPair::DMolarUMolar => (FluidParam::DMolar, FluidParam::UMolar),
        }
    }
}

impl TryFrom<(FluidParam, FluidParam)> for FluidInputPair {
    type Error = strum::ParseError;

    fn try_from(value: (FluidParam, FluidParam)) -> Result<Self, Self::Error> {
        match value {
            (FluidParam::Q, FluidParam::T) | (FluidParam::T, FluidParam::Q) => {
                Ok(FluidInputPair::QT)
            }
            (FluidParam::QMass, FluidParam::T) | (FluidParam::T, FluidParam::QMass) => {
                Ok(FluidInputPair::QMassT)
            }
            (FluidParam::P, FluidParam::Q) | (FluidParam::Q, FluidParam::P) => {
                Ok(FluidInputPair::PQ)
            }
            (FluidParam::P, FluidParam::QMass) | (FluidParam::QMass, FluidParam::P) => {
                Ok(FluidInputPair::PQMass)
            }
            (FluidParam::Q, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::Q) => {
                Ok(FluidInputPair::QSMolar)
            }
            (FluidParam::QMass, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::QMass) => {
                Ok(FluidInputPair::QMassSMolar)
            }
            (FluidParam::Q, FluidParam::SMass) | (FluidParam::SMass, FluidParam::Q) => {
                Ok(FluidInputPair::QSMass)
            }
            (FluidParam::QMass, FluidParam::SMass) | (FluidParam::SMass, FluidParam::QMass) => {
                Ok(FluidInputPair::QMassSMass)
            }
            (FluidParam::HMolar, FluidParam::Q) | (FluidParam::Q, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarQ)
            }
            (FluidParam::HMolar, FluidParam::QMass) | (FluidParam::QMass, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarQMass)
            }
            (FluidParam::HMass, FluidParam::Q) | (FluidParam::Q, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassQ)
            }
            (FluidParam::HMass, FluidParam::QMass) | (FluidParam::QMass, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassQMass)
            }
            (FluidParam::DMolar, FluidParam::Q) | (FluidParam::Q, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarQ)
            }
            (FluidParam::DMolar, FluidParam::QMass) | (FluidParam::QMass, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarQMass)
            }
            (FluidParam::DMass, FluidParam::Q) | (FluidParam::Q, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassQ)
            }
            (FluidParam::DMass, FluidParam::QMass) | (FluidParam::QMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassQMass)
            }
            (FluidParam::P, FluidParam::T) | (FluidParam::T, FluidParam::P) => {
                Ok(FluidInputPair::PT)
            }
            (FluidParam::DMass, FluidParam::T) | (FluidParam::T, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassT)
            }
            (FluidParam::DMolar, FluidParam::T) | (FluidParam::T, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarT)
            }
            (FluidParam::HMolar, FluidParam::T) | (FluidParam::T, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarT)
            }
            (FluidParam::HMass, FluidParam::T) | (FluidParam::T, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassT)
            }
            (FluidParam::SMolar, FluidParam::T) | (FluidParam::T, FluidParam::SMolar) => {
                Ok(FluidInputPair::SMolarT)
            }
            (FluidParam::SMass, FluidParam::T) | (FluidParam::T, FluidParam::SMass) => {
                Ok(FluidInputPair::SMassT)
            }
            (FluidParam::T, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::T) => {
                Ok(FluidInputPair::TUMolar)
            }
            (FluidParam::T, FluidParam::UMass) | (FluidParam::UMass, FluidParam::T) => {
                Ok(FluidInputPair::TUMass)
            }
            (FluidParam::DMass, FluidParam::P) | (FluidParam::P, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassP)
            }
            (FluidParam::DMolar, FluidParam::P) | (FluidParam::P, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarP)
            }
            (FluidParam::HMass, FluidParam::P) | (FluidParam::P, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassP)
            }
            (FluidParam::HMolar, FluidParam::P) | (FluidParam::P, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarP)
            }
            (FluidParam::P, FluidParam::SMass) | (FluidParam::SMass, FluidParam::P) => {
                Ok(FluidInputPair::PSMass)
            }
            (FluidParam::P, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::P) => {
                Ok(FluidInputPair::PSMolar)
            }
            (FluidParam::P, FluidParam::UMass) | (FluidParam::UMass, FluidParam::P) => {
                Ok(FluidInputPair::PUMass)
            }
            (FluidParam::P, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::P) => {
                Ok(FluidInputPair::PUMolar)
            }
            (FluidParam::HMass, FluidParam::SMass) | (FluidParam::SMass, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassSMass)
            }
            (FluidParam::HMolar, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarSMolar)
            }
            (FluidParam::SMass, FluidParam::UMass) | (FluidParam::UMass, FluidParam::SMass) => {
                Ok(FluidInputPair::SMassUMass)
            }
            (FluidParam::SMolar, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::SMolar) => {
                Ok(FluidInputPair::SMolarUMolar)
            }
            (FluidParam::DMass, FluidParam::HMass) | (FluidParam::HMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassHMass)
            }
            (FluidParam::DMolar, FluidParam::HMolar) | (FluidParam::HMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarHMolar)
            }
            (FluidParam::DMass, FluidParam::SMass) | (FluidParam::SMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassSMass)
            }
            (FluidParam::DMolar, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarSMolar)
            }
            (FluidParam::DMass, FluidParam::UMass) | (FluidParam::UMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassUMass)
            }
            (FluidParam::DMolar, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarUMolar)
            }
            _ => Err(strum::ParseError::VariantNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{FluidInputPair::*, FluidParam::*, *};

    #[rstest]
    #[case(QT, 1)]
    #[case(QMassT, 2)]
    #[case(PQ, 3)]
    #[case(PQMass, 4)]
    #[case(QSMolar, 5)]
    #[case(QMassSMolar, 6)]
    #[case(QSMass, 7)]
    #[case(QMassSMass, 8)]
    #[case(HMolarQ, 9)]
    #[case(HMolarQMass, 10)]
    #[case(HMassQ, 11)]
    #[case(HMassQMass, 12)]
    #[case(DMolarQ, 13)]
    #[case(DMolarQMass, 14)]
    #[case(DMassQ, 15)]
    #[case(DMassQMass, 16)]
    #[case(PT, 17)]
    #[case(DMassT, 18)]
    #[case(DMolarT, 19)]
    #[case(HMolarT, 20)]
    #[case(HMassT, 21)]
    #[case(SMolarT, 22)]
    #[case(SMassT, 23)]
    #[case(TUMolar, 24)]
    #[case(TUMass, 25)]
    #[case(DMassP, 26)]
    #[case(DMolarP, 27)]
    #[case(HMassP, 28)]
    #[case(HMolarP, 29)]
    #[case(PSMass, 30)]
    #[case(PSMolar, 31)]
    #[case(PUMass, 32)]
    #[case(PUMolar, 33)]
    #[case(HMassSMass, 34)]
    #[case(HMolarSMolar, 35)]
    #[case(SMassUMass, 36)]
    #[case(SMolarUMolar, 37)]
    #[case(DMassHMass, 38)]
    #[case(DMolarHMolar, 39)]
    #[case(DMassSMass, 40)]
    #[case(DMolarSMolar, 41)]
    #[case(DMassUMass, 42)]
    #[case(DMolarUMolar, 43)]
    fn into_u8(#[case] sut: FluidInputPair, #[case] expected: u8) {
        // When
        let res: u8 = sut.into();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(QT, (Q, T))]
    #[case(QMassT, (QMass, T))]
    #[case(PQ, (P, Q))]
    #[case(PQMass, (P, QMass))]
    #[case(QSMolar, (Q, SMolar))]
    #[case(QMassSMolar, (QMass, SMolar))]
    #[case(QSMass, (Q, SMass))]
    #[case(QMassSMass, (QMass, SMass))]
    #[case(HMolarQ, (HMolar, Q))]
    #[case(HMolarQMass, (HMolar, QMass))]
    #[case(HMassQ, (HMass, Q))]
    #[case(HMassQMass, (HMass, QMass))]
    #[case(DMolarQ, (DMolar, Q))]
    #[case(DMolarQMass, (DMolar, QMass))]
    #[case(DMassQ, (DMass, Q))]
    #[case(DMassQMass, (DMass, QMass))]
    #[case(PT, (P, T))]
    #[case(DMassT, (DMass, T))]
    #[case(DMolarT, (DMolar, T))]
    #[case(HMolarT, (HMolar, T))]
    #[case(HMassT, (HMass, T))]
    #[case(SMolarT, (SMolar, T))]
    #[case(SMassT, (SMass, T))]
    #[case(TUMolar, (T, UMolar))]
    #[case(TUMass, (T, UMass))]
    #[case(DMassP, (DMass, P))]
    #[case(DMolarP, (DMolar, P))]
    #[case(HMassP, (HMass, P))]
    #[case(HMolarP, (HMolar, P))]
    #[case(PSMass, (P, SMass))]
    #[case(PSMolar, (P, SMolar))]
    #[case(PUMass, (P, UMass))]
    #[case(PUMolar, (P, UMolar))]
    #[case(HMassSMass, (HMass, SMass))]
    #[case(HMolarSMolar, (HMolar, SMolar))]
    #[case(SMassUMass, (SMass, UMass))]
    #[case(SMolarUMolar, (SMolar, UMolar))]
    #[case(DMassHMass, (DMass, HMass))]
    #[case(DMolarHMolar, (DMolar, HMolar))]
    #[case(DMassSMass, (DMass, SMass))]
    #[case(DMolarSMolar, (DMolar, SMolar))]
    #[case(DMassUMass, (DMass, UMass))]
    #[case(DMolarUMolar, (DMolar, UMolar))]
    fn into_params(#[case] sut: FluidInputPair, #[case] expected: (FluidParam, FluidParam)) {
        // When
        let res: (FluidParam, FluidParam) = sut.into();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case((Q, T), QT)]
    #[case((T, Q), QT)]
    #[case((QMass, T), QMassT)]
    #[case((T, QMass), QMassT)]
    #[case((P, Q), PQ)]
    #[case((Q, P), PQ)]
    #[case((P, QMass), PQMass)]
    #[case((QMass, P), PQMass)]
    #[case((Q, SMolar), QSMolar)]
    #[case((SMolar, Q), QSMolar)]
    #[case((QMass, SMolar), QMassSMolar)]
    #[case((SMolar, QMass), QMassSMolar)]
    #[case((Q, SMass), QSMass)]
    #[case((SMass, Q), QSMass)]
    #[case((QMass, SMass), QMassSMass)]
    #[case((SMass, QMass), QMassSMass)]
    #[case((HMolar, Q), HMolarQ)]
    #[case((Q, HMolar), HMolarQ)]
    #[case((HMolar, QMass), HMolarQMass)]
    #[case((QMass, HMolar), HMolarQMass)]
    #[case((HMass, Q), HMassQ)]
    #[case((Q, HMass), HMassQ)]
    #[case((HMass, QMass), HMassQMass)]
    #[case((QMass, HMass), HMassQMass)]
    #[case((DMolar, Q), DMolarQ)]
    #[case((Q, DMolar), DMolarQ)]
    #[case((DMolar, QMass), DMolarQMass)]
    #[case((QMass, DMolar), DMolarQMass)]
    #[case((DMass, Q), DMassQ)]
    #[case((Q, DMass), DMassQ)]
    #[case((DMass, QMass), DMassQMass)]
    #[case((QMass, DMass), DMassQMass)]
    #[case((P, T), PT)]
    #[case((T, P), PT)]
    #[case((DMass, T), DMassT)]
    #[case((T, DMass), DMassT)]
    #[case((DMolar, T), DMolarT)]
    #[case((T, DMolar), DMolarT)]
    #[case((HMolar, T), HMolarT)]
    #[case((T, HMolar), HMolarT)]
    #[case((HMass, T), HMassT)]
    #[case((T, HMass), HMassT)]
    #[case((SMolar, T), SMolarT)]
    #[case((T, SMolar), SMolarT)]
    #[case((SMass, T), SMassT)]
    #[case((T, SMass), SMassT)]
    #[case((T, UMolar), TUMolar)]
    #[case((UMolar, T), TUMolar)]
    #[case((T, UMass), TUMass)]
    #[case((UMass, T), TUMass)]
    #[case((DMass, P), DMassP)]
    #[case((P, DMass), DMassP)]
    #[case((DMolar, P), DMolarP)]
    #[case((P, DMolar), DMolarP)]
    #[case((HMass, P), HMassP)]
    #[case((P, HMass), HMassP)]
    #[case((HMolar, P), HMolarP)]
    #[case((P, HMolar), HMolarP)]
    #[case((P, SMass), PSMass)]
    #[case((SMass, P), PSMass)]
    #[case((P, SMolar), PSMolar)]
    #[case((SMolar, P), PSMolar)]
    #[case((P, UMass), PUMass)]
    #[case((UMass, P), PUMass)]
    #[case((P, UMolar), PUMolar)]
    #[case((UMolar, P), PUMolar)]
    #[case((HMass, SMass), HMassSMass)]
    #[case((SMass, HMass), HMassSMass)]
    #[case((HMolar, SMolar), HMolarSMolar)]
    #[case((SMolar, HMolar), HMolarSMolar)]
    #[case((SMass, UMass), SMassUMass)]
    #[case((UMass, SMass), SMassUMass)]
    #[case((SMolar, UMolar), SMolarUMolar)]
    #[case((UMolar, SMolar), SMolarUMolar)]
    #[case((DMass, HMass), DMassHMass)]
    #[case((HMass, DMass), DMassHMass)]
    #[case((DMolar, HMolar), DMolarHMolar)]
    #[case((HMolar, DMolar), DMolarHMolar)]
    #[case((DMass, SMass), DMassSMass)]
    #[case((SMass, DMass), DMassSMass)]
    #[case((DMolar, SMolar), DMolarSMolar)]
    #[case((SMolar, DMolar), DMolarSMolar)]
    #[case((DMass, UMass), DMassUMass)]
    #[case((UMass, DMass), DMassUMass)]
    #[case((DMolar, UMolar), DMolarUMolar)]
    #[case((UMolar, DMolar), DMolarUMolar)]
    fn try_from_valid_params(
        #[case] valid: (FluidParam, FluidParam),
        #[case] expected: FluidInputPair,
    ) {
        // When
        let res = FluidInputPair::try_from(valid).unwrap();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case((CvMass, CpMass))]
    #[case((Phase, DMolar))]
    #[case((Tau, Delta))]
    fn try_from_invalid_params(#[case] invalid: (FluidParam, FluidParam)) {
        // When
        let res = FluidInputPair::try_from(invalid);

        // Then
        assert!(res.is_err());
    }
}
