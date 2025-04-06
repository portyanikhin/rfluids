use super::{HumidAir, StateResult, requests::HumidAirUpdateRequest};
use crate::{
    io::{Input, humid_air_input::HumidAirInput},
    state_variant::StateVariant,
};

impl<S: StateVariant> HumidAir<S> {
    pub(crate) fn inner_update(
        &mut self,
        input1: HumidAirInput,
        input2: HumidAirInput,
        input3: HumidAirInput,
    ) -> StateResult<()> {
        let request: HumidAirUpdateRequest = (input1, input2, input3).try_into()?;
        self.outputs.clear();
        self.outputs.insert(input1.key(), Ok(input1.si_value()));
        self.outputs.insert(input2.key(), Ok(input2.si_value()));
        self.outputs.insert(input3.key(), Ok(input3.si_value()));
        self.update_request = Some(request);
        Ok(())
    }
}
