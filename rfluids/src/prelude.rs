//! Prelude imports.

pub use fluid::*;
pub use native::*;

pub mod fluid {
    //! Prelude imports of [`fluid`](crate::fluid) module.

    pub use crate::fluid::*;
    pub use crate::io::{fluid_input, FluidInput, Phase};
    pub use crate::state_variant::*;
    pub use crate::substance::*;
    pub use crate::uom::si::f64::{
        AvailableEnergy, DynamicViscosity, Force, Length, MassDensity, MolarConcentration,
        MolarEnergy, MolarHeatCapacity, Pressure, Ratio, SpecificHeatCapacity, SpecificVolume,
        TemperatureCoefficient, ThermalConductivity, ThermodynamicTemperature, Velocity,
    };
    pub use crate::uom_ext::kinematic_viscosity::KinematicViscosity;
    pub use crate::uom_ext::pressure_coefficient::PressureCoefficient;
    pub use crate::uom_ext::surface_tension::SurfaceTension;
}

pub mod native {
    //! Prelude imports of [`native`](crate::native) module.

    pub use crate::io::{FluidInputPair, FluidParam, FluidTrivialParam, Phase};
    pub use crate::native::*;
    pub use crate::substance::*;
}
