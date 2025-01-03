use crate::native::{CoolPropError, Parameter};

/// CoolProp input pairs
/// (for use in [`AbstractState::update`](crate::native::AbstractState::update)).
///
/// # Examples
///
/// How to parse [`InputPair`] from two [`Parameter`](Parameter)s:
///
/// ```
/// use fluids_rs::native::{InputPair, Parameter};
///
/// let result = InputPair::try_from((Parameter::T, Parameter::P)).unwrap();
/// assert_eq!(result, InputPair::PT);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputPair {
    /// Vapor quality _(dimensionless, from 0 to 1)_, temperature _(K)_.
    QT = 1,

    /// Pressure _(Pa)_, vapor quality _(dimensionless, from 0 to 1)_.
    PQ = 2,

    /// Vapor quality _(dimensionless, from 0 to 1)_, molar specific entropy _(J/mol/K)_.
    QSMolar = 3,

    /// Vapor quality _(dimensionless, from 0 to 1)_, mass specific entropy _(J/kg/K)_.
    QSMass = 4,

    /// Molar specific enthalpy _(J/mol)_, vapor quality _(dimensionless, from 0 to 1)_.
    HMolarQ = 5,

    /// Mass specific enthalpy _(J/kg)_, vapor quality _(dimensionless, from 0 to 1)_.
    HMassQ = 6,

    /// Molar density _(mol/m3)_, vapor quality _(dimensionless, from 0 to 1)_.
    DMolarQ = 7,

    /// Mass density _(kg/m3)_, vapor quality _(dimensionless, from 0 to 1)_.
    DMassQ = 8,

    /// Pressure _(Pa)_, temperature _(K)_.
    PT = 9,

    /// Mass density _(kg/m3)_, temperature _(K)_.
    DMassT = 10,

    /// Molar density _(mol/m3)_, temperature _(K)_.
    DMolarT = 11,

    /// Molar specific enthalpy _(J/mol)_, temperature _(K)_.
    HMolarT = 12,

    /// Mass specific enthalpy _(J/kg)_, temperature _(K)_.
    HMassT = 13,

    /// Molar specific entropy _(J/mol/K)_, temperature _(K)_.
    SMolarT = 14,

    /// Mass specific entropy _(J/kg/K)_, temperature _(K)_.
    SMassT = 15,

    /// Temperature _(K)_, molar specific internal energy _(J/mol)_.
    TUMolar = 16,

    /// Temperature _(K)_, mass specific internal energy _(J/kg)_.
    TUMass = 17,

    /// Mass density _(kg/m3)_, pressure _(Pa)_.
    DMassP = 18,

    /// Molar density _(mol/m3)_, pressure _(Pa)_.
    DMolarP = 19,

    /// Mass specific enthalpy _(J/kg)_, pressure _(Pa)_.
    HMassP = 20,

    /// Molar specific enthalpy _(J/mol)_, pressure _(Pa)_.
    HMolarP = 21,

    /// Pressure _(Pa)_, mass specific entropy _(J/kg/K)_.
    PSMass = 22,

    /// Pressure _(Pa)_, molar specific entropy _(J/mol/K)_.
    PSMolar = 23,

    /// Pressure _(Pa)_, mass specific internal energy _(J/kg)_.
    PUMass = 24,

    /// Pressure _(Pa)_, molar specific internal energy _(J/mol)_.
    PUMolar = 25,

    /// Mass specific enthalpy _(J/kg)_, mass specific entropy _(J/kg/K)_.
    HMassSMass = 26,

    /// Molar specific enthalpy _(J/mol)_, molar specific entropy _(J/mol/K)_.
    HMolarSMolar = 27,

    /// Mass specific entropy _(J/kg/K)_, mass specific internal energy _(J/kg)_.
    SMassUMass = 28,

    /// Molar specific entropy _(J/mol/K)_, molar specific internal energy _(J/mol)_.
    SMolarUMolar = 29,

    /// Mass density _(kg/m3)_, mass specific enthalpy _(J/kg)_.
    DMassHMass = 30,

    /// Molar density _(mol/m3)_, molar specific enthalpy _(J/mol)_.
    DMolarHMolar = 31,

    /// Mass density _(kg/m3)_, mass specific entropy _(J/kg/K)_.
    DMassSMass = 32,

    /// Molar density _(mol/m3)_, molar specific entropy _(J/mol/K)_.
    DMolarSMolar = 33,

    /// Mass density _(kg/m3)_, mass specific internal energy _(J/kg)_.
    DMassUMass = 34,

    /// Molar density _(mol/m3)_, molar specific internal energy _(J/mol)_.
    DMolarUMolar = 35,
}

impl TryFrom<(Parameter, Parameter)> for InputPair {
    type Error = CoolPropError;

    fn try_from(value: (Parameter, Parameter)) -> Result<Self, Self::Error> {
        match value {
            (Parameter::Q, Parameter::T) | (Parameter::T, Parameter::Q) => Ok(InputPair::QT),
            (Parameter::P, Parameter::Q) | (Parameter::Q, Parameter::P) => Ok(InputPair::PQ),
            (Parameter::Q, Parameter::SMolar) | (Parameter::SMolar, Parameter::Q) => {
                Ok(InputPair::QSMolar)
            }
            (Parameter::Q, Parameter::SMass) | (Parameter::SMass, Parameter::Q) => {
                Ok(InputPair::QSMass)
            }
            (Parameter::HMolar, Parameter::Q) | (Parameter::Q, Parameter::HMolar) => {
                Ok(InputPair::HMolarQ)
            }
            (Parameter::HMass, Parameter::Q) | (Parameter::Q, Parameter::HMass) => {
                Ok(InputPair::HMassQ)
            }
            (Parameter::DMolar, Parameter::Q) | (Parameter::Q, Parameter::DMolar) => {
                Ok(InputPair::DMolarQ)
            }
            (Parameter::DMass, Parameter::Q) | (Parameter::Q, Parameter::DMass) => {
                Ok(InputPair::DMassQ)
            }
            (Parameter::P, Parameter::T) | (Parameter::T, Parameter::P) => Ok(InputPair::PT),
            (Parameter::DMass, Parameter::T) | (Parameter::T, Parameter::DMass) => {
                Ok(InputPair::DMassT)
            }
            (Parameter::DMolar, Parameter::T) | (Parameter::T, Parameter::DMolar) => {
                Ok(InputPair::DMolarT)
            }
            (Parameter::HMolar, Parameter::T) | (Parameter::T, Parameter::HMolar) => {
                Ok(InputPair::HMolarT)
            }
            (Parameter::HMass, Parameter::T) | (Parameter::T, Parameter::HMass) => {
                Ok(InputPair::HMassT)
            }
            (Parameter::SMolar, Parameter::T) | (Parameter::T, Parameter::SMolar) => {
                Ok(InputPair::SMolarT)
            }
            (Parameter::SMass, Parameter::T) | (Parameter::T, Parameter::SMass) => {
                Ok(InputPair::SMassT)
            }
            (Parameter::T, Parameter::UMolar) | (Parameter::UMolar, Parameter::T) => {
                Ok(InputPair::TUMolar)
            }
            (Parameter::T, Parameter::UMass) | (Parameter::UMass, Parameter::T) => {
                Ok(InputPair::TUMass)
            }
            (Parameter::DMass, Parameter::P) | (Parameter::P, Parameter::DMass) => {
                Ok(InputPair::DMassP)
            }
            (Parameter::DMolar, Parameter::P) | (Parameter::P, Parameter::DMolar) => {
                Ok(InputPair::DMolarP)
            }
            (Parameter::HMass, Parameter::P) | (Parameter::P, Parameter::HMass) => {
                Ok(InputPair::HMassP)
            }
            (Parameter::HMolar, Parameter::P) | (Parameter::P, Parameter::HMolar) => {
                Ok(InputPair::HMolarP)
            }
            (Parameter::P, Parameter::SMass) | (Parameter::SMass, Parameter::P) => {
                Ok(InputPair::PSMass)
            }
            (Parameter::P, Parameter::SMolar) | (Parameter::SMolar, Parameter::P) => {
                Ok(InputPair::PSMolar)
            }
            (Parameter::P, Parameter::UMass) | (Parameter::UMass, Parameter::P) => {
                Ok(InputPair::PUMass)
            }
            (Parameter::P, Parameter::UMolar) | (Parameter::UMolar, Parameter::P) => {
                Ok(InputPair::PUMolar)
            }
            (Parameter::HMass, Parameter::SMass) | (Parameter::SMass, Parameter::HMass) => {
                Ok(InputPair::HMassSMass)
            }
            (Parameter::HMolar, Parameter::SMolar) | (Parameter::SMolar, Parameter::HMolar) => {
                Ok(InputPair::HMolarSMolar)
            }
            (Parameter::SMass, Parameter::UMass) | (Parameter::UMass, Parameter::SMass) => {
                Ok(InputPair::SMassUMass)
            }
            (Parameter::SMolar, Parameter::UMolar) | (Parameter::UMolar, Parameter::SMolar) => {
                Ok(InputPair::SMolarUMolar)
            }
            (Parameter::DMass, Parameter::HMass) | (Parameter::HMass, Parameter::DMass) => {
                Ok(InputPair::DMassHMass)
            }
            (Parameter::DMolar, Parameter::HMolar) | (Parameter::HMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarHMolar)
            }
            (Parameter::DMass, Parameter::SMass) | (Parameter::SMass, Parameter::DMass) => {
                Ok(InputPair::DMassSMass)
            }
            (Parameter::DMolar, Parameter::SMolar) | (Parameter::SMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarSMolar)
            }
            (Parameter::DMass, Parameter::UMass) | (Parameter::UMass, Parameter::DMass) => {
                Ok(InputPair::DMassUMass)
            }
            (Parameter::DMolar, Parameter::UMolar) | (Parameter::UMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarUMolar)
            }
            (input1, input2) => Err(CoolPropError(format!(
                "Specified parameters ('{:?}', '{:?}') has no matching input pair!",
                input1, input2
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case((Parameter::Q, Parameter::T), InputPair::QT)]
    #[case((Parameter::T, Parameter::Q), InputPair::QT)]
    #[case((Parameter::P, Parameter::Q), InputPair::PQ)]
    #[case((Parameter::Q, Parameter::P), InputPair::PQ)]
    #[case((Parameter::Q, Parameter::SMolar), InputPair::QSMolar)]
    #[case((Parameter::SMolar, Parameter::Q), InputPair::QSMolar)]
    #[case((Parameter::Q, Parameter::SMass), InputPair::QSMass)]
    #[case((Parameter::SMass, Parameter::Q), InputPair::QSMass)]
    #[case((Parameter::HMolar, Parameter::Q), InputPair::HMolarQ)]
    #[case((Parameter::Q, Parameter::HMolar), InputPair::HMolarQ)]
    #[case((Parameter::HMass, Parameter::Q), InputPair::HMassQ)]
    #[case((Parameter::Q, Parameter::HMass), InputPair::HMassQ)]
    #[case((Parameter::DMolar, Parameter::Q), InputPair::DMolarQ)]
    #[case((Parameter::Q, Parameter::DMolar), InputPair::DMolarQ)]
    #[case((Parameter::DMass, Parameter::Q), InputPair::DMassQ)]
    #[case((Parameter::Q, Parameter::DMass), InputPair::DMassQ)]
    #[case((Parameter::P, Parameter::T), InputPair::PT)]
    #[case((Parameter::T, Parameter::P), InputPair::PT)]
    #[case((Parameter::DMass, Parameter::T), InputPair::DMassT)]
    #[case((Parameter::T, Parameter::DMass), InputPair::DMassT)]
    #[case((Parameter::DMolar, Parameter::T), InputPair::DMolarT)]
    #[case((Parameter::T, Parameter::DMolar), InputPair::DMolarT)]
    #[case((Parameter::HMolar, Parameter::T), InputPair::HMolarT)]
    #[case((Parameter::T, Parameter::HMolar), InputPair::HMolarT)]
    #[case((Parameter::HMass, Parameter::T), InputPair::HMassT)]
    #[case((Parameter::T, Parameter::HMass), InputPair::HMassT)]
    #[case((Parameter::SMolar, Parameter::T), InputPair::SMolarT)]
    #[case((Parameter::T, Parameter::SMolar), InputPair::SMolarT)]
    #[case((Parameter::SMass, Parameter::T), InputPair::SMassT)]
    #[case((Parameter::T, Parameter::SMass), InputPair::SMassT)]
    #[case((Parameter::T, Parameter::UMolar), InputPair::TUMolar)]
    #[case((Parameter::UMolar, Parameter::T), InputPair::TUMolar)]
    #[case((Parameter::T, Parameter::UMass), InputPair::TUMass)]
    #[case((Parameter::UMass, Parameter::T), InputPair::TUMass)]
    #[case((Parameter::DMass, Parameter::P), InputPair::DMassP)]
    #[case((Parameter::P, Parameter::DMass), InputPair::DMassP)]
    #[case((Parameter::DMolar, Parameter::P), InputPair::DMolarP)]
    #[case((Parameter::P, Parameter::DMolar), InputPair::DMolarP)]
    #[case((Parameter::HMass, Parameter::P), InputPair::HMassP)]
    #[case((Parameter::P, Parameter::HMass), InputPair::HMassP)]
    #[case((Parameter::HMolar, Parameter::P), InputPair::HMolarP)]
    #[case((Parameter::P, Parameter::HMolar), InputPair::HMolarP)]
    #[case((Parameter::P, Parameter::SMass), InputPair::PSMass)]
    #[case((Parameter::SMass, Parameter::P), InputPair::PSMass)]
    #[case((Parameter::P, Parameter::SMolar), InputPair::PSMolar)]
    #[case((Parameter::SMolar, Parameter::P), InputPair::PSMolar)]
    #[case((Parameter::P, Parameter::UMass), InputPair::PUMass)]
    #[case((Parameter::UMass, Parameter::P), InputPair::PUMass)]
    #[case((Parameter::P, Parameter::UMolar), InputPair::PUMolar)]
    #[case((Parameter::UMolar, Parameter::P), InputPair::PUMolar)]
    #[case((Parameter::HMass, Parameter::SMass), InputPair::HMassSMass)]
    #[case((Parameter::SMass, Parameter::HMass), InputPair::HMassSMass)]
    #[case((Parameter::HMolar, Parameter::SMolar), InputPair::HMolarSMolar)]
    #[case((Parameter::SMolar, Parameter::HMolar), InputPair::HMolarSMolar)]
    #[case((Parameter::SMass, Parameter::UMass), InputPair::SMassUMass)]
    #[case((Parameter::UMass, Parameter::SMass), InputPair::SMassUMass)]
    #[case((Parameter::SMolar, Parameter::UMolar), InputPair::SMolarUMolar)]
    #[case((Parameter::UMolar, Parameter::SMolar), InputPair::SMolarUMolar)]
    #[case((Parameter::DMass, Parameter::HMass), InputPair::DMassHMass)]
    #[case((Parameter::HMass, Parameter::DMass), InputPair::DMassHMass)]
    #[case((Parameter::DMolar, Parameter::HMolar), InputPair::DMolarHMolar)]
    #[case((Parameter::HMolar, Parameter::DMolar), InputPair::DMolarHMolar)]
    #[case((Parameter::DMass, Parameter::SMass), InputPair::DMassSMass)]
    #[case((Parameter::SMass, Parameter::DMass), InputPair::DMassSMass)]
    #[case((Parameter::DMolar, Parameter::SMolar), InputPair::DMolarSMolar)]
    #[case((Parameter::SMolar, Parameter::DMolar), InputPair::DMolarSMolar)]
    #[case((Parameter::DMass, Parameter::UMass), InputPair::DMassUMass)]
    #[case((Parameter::UMass, Parameter::DMass), InputPair::DMassUMass)]
    #[case((Parameter::DMolar, Parameter::UMolar), InputPair::DMolarUMolar)]
    #[case((Parameter::UMolar, Parameter::DMolar), InputPair::DMolarUMolar)]
    fn input_pair_try_from_two_valid_parameters_returns_ok(
        #[case] valid_parameters: (Parameter, Parameter),
        #[case] expected: InputPair,
    ) {
        let result = InputPair::try_from(valid_parameters);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case((Parameter::TCritical, Parameter::CpMass))]
    #[case((Parameter::Phase, Parameter::DMolar))]
    #[case((Parameter::GWP100, Parameter::ODP))]
    fn input_pair_try_from_two_invalid_parameters_returns_err(
        #[case] invalid_parameters: (Parameter, Parameter),
    ) {
        let result = InputPair::try_from(invalid_parameters);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Specified parameters ('{:?}', '{:?}') has no matching input pair!",
                invalid_parameters.0, invalid_parameters.1
            )
        );
    }
}
