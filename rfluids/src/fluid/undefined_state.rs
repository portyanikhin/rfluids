use crate::fluid::Fluid;
use crate::native::AbstractState;
use crate::substance::BackendName;
use crate::uom::si::f64::Ratio;
use crate::UndefinedState;
use std::collections::HashMap;
use std::marker::PhantomData;

impl<T> Fluid<T, UndefinedState>
where
    T: AsRef<str> + BackendName + Copy,
{
    pub(crate) fn new(substance: T, fraction: Option<Ratio>) -> Self {
        Self {
            substance,
            fraction,
            backend: AbstractState::new(substance.backend_name(), substance).unwrap(),
            trivial_params_cache: HashMap::new(),
            params_cache: HashMap::new(),
            state: PhantomData,
        }
    }
}
