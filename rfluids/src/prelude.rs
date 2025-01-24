//! Prelude imports.

pub use fluid::*;
pub use native::*;

pub mod fluid {
    //! Prelude imports of [`fluid`](crate::fluid) module.

    pub use crate::fluid::*;
    pub use crate::io::{FluidInput, Phase};
    pub use crate::state_variant::*;
    pub use crate::substance::*;
    pub use crate::uom::si::f64::{
        AvailableEnergy, MassDensity, MolarConcentration, MolarEnergy, MolarHeatCapacity, Pressure,
        Ratio, SpecificHeatCapacity, ThermodynamicTemperature,
    };
}

pub mod native {
    //! Prelude imports of [`native`](crate::native) module.

    pub use crate::fluid::BackendName;
    pub use crate::io::{FluidInputPair, FluidParam, FluidTrivialParam, Phase};
    pub use crate::native::*;
    pub use crate::substance::*;
}
