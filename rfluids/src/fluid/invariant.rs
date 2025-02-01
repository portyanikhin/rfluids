use crate::error::FluidUpdateError;
use crate::fluid::Fluid;
use crate::io::FluidInput;
use crate::state_variant::StateVariant;

use super::requests::FluidUpdateRequest;

impl<S: StateVariant> Fluid<S> {
    pub(crate) fn inner_update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<(), FluidUpdateError> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        self.backend.update(request.0, request.1, request.2)?;
        self.outputs.clear();
        self.outputs.insert(input1.key(), input1.si_value());
        self.outputs.insert(input2.key(), input2.si_value());
        self.update_request = Some(request);
        Ok(())
    }
}
