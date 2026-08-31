//! Raw FFI bindings to `CoolProp`.
//!
//! # See Also
//!
//! - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_2_cool_prop_lib_8h.html)

#![allow(
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clippy::pedantic
)]

#[cfg(all(feature = "static-link", feature = "regen-bindings"))]
pub mod bindings_generated_static {
    #![allow(
        dead_code,
        missing_docs,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unsafe_op_in_unsafe_fn
    )]
    include!(concat!(env!("OUT_DIR"), "/bindings_static.rs"));
}

#[cfg(all(feature = "static-link", not(feature = "regen-bindings")))]
pub mod bindings_generated_static {
    #![allow(
        dead_code,
        missing_docs,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unsafe_op_in_unsafe_fn
    )]
    include!("bindings_generated_static.rs");
}

#[cfg(feature = "static-link")]
macro_rules! generate_static_coolprop {
    (
        $(
            pub fn $func_name:ident(
                $( $arg_name:ident : $arg_ty:ty ),* $(,)?
            ) $(-> $ret_ty:ty)?;
        )*
    ) => {
        pub struct CoolProp;

        impl CoolProp {
            /// Statically linked functions are resolved at link time, so this can never fail.
            /// The fallible signature only exists so call sites don't need to differ from the
            /// dynamic constructor's.
            pub fn new() -> Result<Self, ()> {
                Ok(CoolProp)
            }

            $(
                #[inline]
                pub unsafe fn $func_name(
                    &self,
                    $( $arg_name : $arg_ty ),*
                ) $(-> $ret_ty)? {
                    self::bindings_generated_static::$func_name($( $arg_name ),*)
                }
            )*
        }
    };
}

#[cfg(feature = "static-link")]
generate_static_coolprop! {
    pub fn Props1SI(FluidName: *const ::core::ffi::c_char, Output: *const ::core::ffi::c_char) -> f64;
    pub fn Props1SImulti(Outputs: *const ::core::ffi::c_char, backend: *mut ::core::ffi::c_char, FluidNames: *const ::core::ffi::c_char, fractions: *const f64, length_fractions: ::core::ffi::c_long, result: *mut f64, resdim1: *mut ::core::ffi::c_long);
    pub fn PropsSI(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: f64, Name2: *const ::core::ffi::c_char, Prop2: f64, FluidName: *const ::core::ffi::c_char) -> f64;
    pub fn PropsSImulti(Outputs: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: *mut f64, size_Prop1: ::core::ffi::c_long, Name2: *const ::core::ffi::c_char, Prop2: *mut f64, size_Prop2: ::core::ffi::c_long, backend: *mut ::core::ffi::c_char, FluidNames: *const ::core::ffi::c_char, fractions: *const f64, length_fractions: ::core::ffi::c_long, result: *mut f64, resdim1: *mut ::core::ffi::c_long, resdim2: *mut ::core::ffi::c_long);
    pub fn PhaseSI(Name1: *const ::core::ffi::c_char, Prop1: f64, Name2: *const ::core::ffi::c_char, Prop2: f64, FluidName: *const ::core::ffi::c_char, phase: *mut ::core::ffi::c_char, n: ::core::ffi::c_int) -> ::core::ffi::c_long;
    pub fn get_global_param_string(param: *const ::core::ffi::c_char, Output: *mut ::core::ffi::c_char, n: ::core::ffi::c_int) -> ::core::ffi::c_long;
    pub fn get_parameter_information_string(param: *const ::core::ffi::c_char, Output: *mut ::core::ffi::c_char, n: ::core::ffi::c_int) -> ::core::ffi::c_long;
    pub fn get_fluid_param_string(fluid: *const ::core::ffi::c_char, param: *const ::core::ffi::c_char, Output: *mut ::core::ffi::c_char, n: ::core::ffi::c_int) -> ::core::ffi::c_long;
    pub fn get_fluid_param_string_len(fluid: *const ::core::ffi::c_char, param: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    pub fn set_config_string(key: *const ::core::ffi::c_char, val: *const ::core::ffi::c_char);
    pub fn set_config_double(key: *const ::core::ffi::c_char, val: f64);
    pub fn set_config_bool(key: *const ::core::ffi::c_char, val: bool);
    pub fn set_departure_functions(string_data: *const ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn set_reference_stateS(Ref: *const ::core::ffi::c_char, reference_state: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn set_reference_stateD(Ref: *const ::core::ffi::c_char, T: f64, rhomolar: f64, hmolar0: f64, smolar0: f64) -> ::core::ffi::c_int;
    pub fn propssi_(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: *const f64, Name2: *const ::core::ffi::c_char, Prop2: *const f64, FluidName: *const ::core::ffi::c_char, output: *mut f64);
    pub fn F2K(T_F: f64) -> f64;
    pub fn K2F(T_K: f64) -> f64;
    pub fn get_param_index(param: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    pub fn get_input_pair_index(pair: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    pub fn redirect_stdout(file: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    pub fn get_debug_level() -> ::core::ffi::c_int;
    pub fn set_debug_level(level: ::core::ffi::c_int);
    pub fn saturation_ancillary(fluid_name: *const ::core::ffi::c_char, output: *const ::core::ffi::c_char, Q: ::core::ffi::c_int, input: *const ::core::ffi::c_char, value: f64) -> f64;
    pub fn HAPropsSI(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: f64, Name2: *const ::core::ffi::c_char, Prop2: f64, Name3: *const ::core::ffi::c_char, Prop3: f64) -> f64;
    pub fn cair_sat(T: f64) -> f64;
    pub fn hapropssi_(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: *const f64, Name2: *const ::core::ffi::c_char, Prop2: *const f64, Name3: *const ::core::ffi::c_char, Prop3: *const f64, output: *mut f64);
    pub fn HAProps(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: f64, Name2: *const ::core::ffi::c_char, Prop2: f64, Name3: *const ::core::ffi::c_char, Prop3: f64) -> f64;
    pub fn haprops_(Output: *const ::core::ffi::c_char, Name1: *const ::core::ffi::c_char, Prop1: *const f64, Name2: *const ::core::ffi::c_char, Prop2: *const f64, Name3: *const ::core::ffi::c_char, Prop3: *const f64, output: *mut f64);
    pub fn AbstractState_factory(backend: *const ::core::ffi::c_char, fluids: *const ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> ::core::ffi::c_long;
    pub fn AbstractState_fluid_names(handle: ::core::ffi::c_long, fluids: *mut ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_free(handle: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_set_fractions(handle: ::core::ffi::c_long, fractions: *const f64, N: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_mole_fractions(handle: ::core::ffi::c_long, fractions: *mut f64, maxN: ::core::ffi::c_long, N: *mut ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_mole_fractions_satState(handle: ::core::ffi::c_long, saturated_state: *const ::core::ffi::c_char, fractions: *mut f64, maxN: ::core::ffi::c_long, N: *mut ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_fugacity(handle: ::core::ffi::c_long, i: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_get_fugacity_coefficient(handle: ::core::ffi::c_long, i: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_update(handle: ::core::ffi::c_long, input_pair: ::core::ffi::c_long, value1: f64, value2: f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_specify_phase(handle: ::core::ffi::c_long, phase: *const ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_unspecify_phase(handle: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_keyed_output(handle: ::core::ffi::c_long, param: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_first_saturation_deriv(handle: ::core::ffi::c_long, Of: ::core::ffi::c_long, Wrt: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_first_partial_deriv(handle: ::core::ffi::c_long, Of: ::core::ffi::c_long, Wrt: ::core::ffi::c_long, Constant: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_second_two_phase_deriv(handle: ::core::ffi::c_long, Of1: ::core::ffi::c_long, Wrt1: ::core::ffi::c_long, Constant1: ::core::ffi::c_long, Wrt2: ::core::ffi::c_long, Constant2: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_second_partial_deriv(handle: ::core::ffi::c_long, Of1: ::core::ffi::c_long, Wrt1: ::core::ffi::c_long, Constant1: ::core::ffi::c_long, Wrt2: ::core::ffi::c_long, Constant2: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_first_two_phase_deriv_splined(handle: ::core::ffi::c_long, Of: ::core::ffi::c_long, Wrt: ::core::ffi::c_long, Constant: ::core::ffi::c_long, x_end: f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_first_two_phase_deriv(handle: ::core::ffi::c_long, Of: ::core::ffi::c_long, Wrt: ::core::ffi::c_long, Constant: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_update_and_common_out(handle: ::core::ffi::c_long, input_pair: ::core::ffi::c_long, value1: *const f64, value2: *const f64, length: ::core::ffi::c_long, T: *mut f64, p: *mut f64, rhomolar: *mut f64, hmolar: *mut f64, smolar: *mut f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_update_and_1_out(handle: ::core::ffi::c_long, input_pair: ::core::ffi::c_long, value1: *const f64, value2: *const f64, length: ::core::ffi::c_long, output: ::core::ffi::c_long, out: *mut f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_update_and_5_out(handle: ::core::ffi::c_long, input_pair: ::core::ffi::c_long, value1: *const f64, value2: *const f64, length: ::core::ffi::c_long, outputs: *mut ::core::ffi::c_long, out1: *mut f64, out2: *mut f64, out3: *mut f64, out4: *mut f64, out5: *mut f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_set_binary_interaction_double(handle: ::core::ffi::c_long, i: ::core::ffi::c_long, j: ::core::ffi::c_long, parameter: *const ::core::ffi::c_char, value: f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_set_cubic_alpha_C(handle: ::core::ffi::c_long, i: ::core::ffi::c_long, parameter: *const ::core::ffi::c_char, c1: f64, c2: f64, c3: f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_set_fluid_parameter_double(handle: ::core::ffi::c_long, i: ::core::ffi::c_long, parameter: *const ::core::ffi::c_char, value: f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_build_phase_envelope(handle: ::core::ffi::c_long, level: *const ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_phase_envelope_data(handle: ::core::ffi::c_long, length: ::core::ffi::c_long, T: *mut f64, p: *mut f64, rhomolar_vap: *mut f64, rhomolar_liq: *mut f64, x: *mut f64, y: *mut f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_phase_envelope_data_checkedMemory(handle: ::core::ffi::c_long, length: ::core::ffi::c_long, maxComponents: ::core::ffi::c_long, T: *mut f64, p: *mut f64, rhomolar_vap: *mut f64, rhomolar_liq: *mut f64, x: *mut f64, y: *mut f64, actual_length: *mut ::core::ffi::c_long, actual_components: *mut ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_build_spinodal(handle: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_get_spinodal_data(handle: ::core::ffi::c_long, length: ::core::ffi::c_long, tau: *mut f64, delta: *mut f64, M1: *mut f64, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_all_critical_points(handle: ::core::ffi::c_long, length: ::core::ffi::c_long, T: *mut f64, p: *mut f64, rhomolar: *mut f64, stable: *mut ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_keyed_output_satState(handle: ::core::ffi::c_long, saturated_state: *const ::core::ffi::c_char, param: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_backend_name(handle: ::core::ffi::c_long, backend: *mut ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_fluid_param_string(handle: ::core::ffi::c_long, param: *const ::core::ffi::c_char, return_buffer: *mut ::core::ffi::c_char, return_buffer_length: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn AbstractState_phase(handle: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> ::core::ffi::c_int;
    pub fn AbstractState_saturated_liquid_keyed_output(handle: ::core::ffi::c_long, param: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn AbstractState_saturated_vapor_keyed_output(handle: ::core::ffi::c_long, param: ::core::ffi::c_long, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long) -> f64;
    pub fn add_fluids_as_JSON(backend: *const ::core::ffi::c_char, fluidstring: *const ::core::ffi::c_char, errcode: *mut ::core::ffi::c_long, message_buffer: *mut ::core::ffi::c_char, buffer_length: ::core::ffi::c_long);
    pub fn C_is_valid_fluid_string(fluidName: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn C_extract_backend(fluid_string: *const ::core::ffi::c_char, backend: *mut ::core::ffi::c_char, backend_length: ::core::ffi::c_long, fluid: *mut ::core::ffi::c_char, fluid_length: ::core::ffi::c_long) -> ::core::ffi::c_int;
}

#[cfg(all(not(feature = "static-link"), feature = "regen-bindings"))]
include!(concat!(env!("OUT_DIR"), "/bindings_dynamic.rs"));

#[cfg(all(not(feature = "static-link"), not(feature = "regen-bindings")))]
include!("bindings_generated_dynamic.rs");
