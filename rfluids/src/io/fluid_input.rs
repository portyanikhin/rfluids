use super::{FluidParam, Input};

/// [`Fluid`](crate::fluid::Fluid) input parameter with specified value.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::*;
///
/// let pressure = FluidInput::pressure(101_325.0);
/// ```
pub type FluidInput = Input<FluidParam>;

impl FluidInput {
    /// Mass density **\[kg/m³\]**.
    #[must_use]
    pub fn density(value: f64) -> Self {
        Self {
            key: FluidParam::DMass,
            value,
        }
    }

    /// Mass specific enthalpy **\[J/kg\]**.
    #[must_use]
    pub fn enthalpy(value: f64) -> Self {
        Self {
            key: FluidParam::HMass,
            value,
        }
    }

    /// Mass specific entropy **\[J/kg/K\]**.
    #[must_use]
    pub fn entropy(value: f64) -> Self {
        Self {
            key: FluidParam::SMass,
            value,
        }
    }

    /// Mass specific internal energy **\[J/kg\]**.
    #[must_use]
    pub fn internal_energy(value: f64) -> Self {
        Self {
            key: FluidParam::UMass,
            value,
        }
    }

    /// Molar density **\[mol/m³\]**.
    #[must_use]
    pub fn molar_density(value: f64) -> Self {
        Self {
            key: FluidParam::DMolar,
            value,
        }
    }

    /// Molar specific enthalpy **\[J/mol\]**.
    #[must_use]
    pub fn molar_enthalpy(value: f64) -> Self {
        Self {
            key: FluidParam::HMolar,
            value,
        }
    }

    /// Molar specific entropy **\[J/mol/K\]**.
    #[must_use]
    pub fn molar_entropy(value: f64) -> Self {
        Self {
            key: FluidParam::SMolar,
            value,
        }
    }

    /// Molar specific internal energy **\[J/mol\]**.
    #[must_use]
    pub fn molar_internal_energy(value: f64) -> Self {
        Self {
            key: FluidParam::UMolar,
            value,
        }
    }

    /// Pressure **\[Pa\]**.
    #[must_use]
    pub fn pressure(value: f64) -> Self {
        Self {
            key: FluidParam::P,
            value,
        }
    }

    /// Vapor quality **\[dimensionless, from 0 to 1\]**.
    #[must_use]
    pub fn quality(value: f64) -> Self {
        Self {
            key: FluidParam::Q,
            value,
        }
    }

    /// Specific volume **\[m³/kg\]**.
    ///
    /// **NB.** It will be converted to the [`density`](Self::density),
    /// since there is no specific [`FluidParam`] for this.
    #[must_use]
    pub fn specific_volume(value: f64) -> Self {
        Self::density(1.0 / value)
    }

    /// Temperature **\[K\]**.
    #[must_use]
    pub fn temperature(value: f64) -> Self {
        Self {
            key: FluidParam::T,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_input;

    test_input!(FluidInput: density, key: FluidParam::DMass);
    test_input!(FluidInput: enthalpy, key: FluidParam::HMass);
    test_input!(FluidInput: entropy, key: FluidParam::SMass);
    test_input!(FluidInput: internal_energy, key: FluidParam::UMass);
    test_input!(FluidInput: molar_density, key: FluidParam::DMolar);
    test_input!(FluidInput: molar_enthalpy, key: FluidParam::HMolar);
    test_input!(FluidInput: molar_entropy, key: FluidParam::SMolar);
    test_input!(FluidInput: molar_internal_energy, key: FluidParam::UMolar);
    test_input!(FluidInput: pressure, key: FluidParam::P);
    test_input!(FluidInput: quality, key: FluidParam::Q);
    test_input!(FluidInput: specific_volume, key: FluidParam::DMass, reciprocal);
    test_input!(FluidInput: temperature, key: FluidParam::T);
}
