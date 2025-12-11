#ifndef COOLPROPDLL_H
#define COOLPROPDLL_H

#if _WIN64
#define __ISWINDOWS__
#elif _WIN32
#define __ISWINDOWS__
#elif __APPLE__
#define __ISAPPLE__
#elif __linux || __unix || __posix
#define __ISLINUX__
#elif __powerpc__
#define __ISPOWERPC__
#else
#pragma error
#endif

#if defined(COOLPROP_LIB)
#ifndef EXPORT_CODE
#if defined(__ISWINDOWS__)
#define EXPORT_CODE extern "C" __declspec(dllexport)
#else
#define EXPORT_CODE extern "C"
#endif
#endif
#ifndef CONVENTION
#if defined(__ISWINDOWS__)
#define CONVENTION __stdcall
#else
#define CONVENTION
#endif
#endif
#else
#ifndef EXPORT_CODE
#define EXPORT_CODE
#endif
#ifndef CONVENTION
#define CONVENTION
#endif
#endif

#ifndef __cplusplus
#if defined(__STDC_VERSION__)
#if (__STDC_VERSION__ >= 199901L)
#include <stdbool.h>
#endif
#endif
#endif

#if defined(__powerpc__) || defined(EXTERNC)
#undef EXPORT_CODE
#define EXPORT_CODE extern "C"
#endif

#if defined(__powerpc__)
inline void __assert(const char *error) {
    while (1)
        ;
}
#endif

EXPORT_CODE double CONVENTION Props1SI(const char *FluidName, const char *Output);

EXPORT_CODE void CONVENTION Props1SImulti(const char *Outputs, char *backend,
                                          const char *FluidNames, const double *fractions,
                                          const long length_fractions, double *result,
                                          long *resdim1);

EXPORT_CODE double CONVENTION PropsSI(const char *Output, const char *Name1, double Prop1,
                                      const char *Name2, double Prop2, const char *FluidName);

EXPORT_CODE void CONVENTION PropsSImulti(const char *Outputs, const char *Name1, double *Prop1,
                                         const long size_Prop1, const char *Name2, double *Prop2,
                                         const long size_Prop2, char *backend,
                                         const char *FluidNames, const double *fractions,
                                         const long length_fractions, double *result, long *resdim1,
                                         long *resdim2);

EXPORT_CODE long CONVENTION PhaseSI(const char *Name1, double Prop1, const char *Name2,
                                    double Prop2, const char *FluidName, char *phase, int n);

EXPORT_CODE long CONVENTION get_global_param_string(const char *param, char *Output, int n);

EXPORT_CODE long CONVENTION get_parameter_information_string(const char *param, char *Output,
                                                             int n);

EXPORT_CODE long CONVENTION get_fluid_param_string(const char *fluid, const char *param,
                                                   char *Output, int n);

EXPORT_CODE long CONVENTION get_fluid_param_string_len(const char *fluid, const char *param);

EXPORT_CODE void CONVENTION set_config_string(const char *key, const char *val);

EXPORT_CODE void CONVENTION set_config_double(const char *key, const double val);

EXPORT_CODE void CONVENTION set_config_bool(const char *key, const bool val);

EXPORT_CODE void CONVENTION set_departure_functions(const char *string_data, long *errcode,
                                                    char *message_buffer, const long buffer_length);

EXPORT_CODE int CONVENTION set_reference_stateS(const char *Ref, const char *reference_state);

EXPORT_CODE int CONVENTION set_reference_stateD(const char *Ref, double T, double rhomolar,
                                                double hmolar0, double smolar0);

EXPORT_CODE void CONVENTION propssi_(const char *Output, const char *Name1, const double *Prop1,
                                     const char *Name2, const double *Prop2, const char *FluidName,
                                     double *output);

EXPORT_CODE double CONVENTION F2K(double T_F);

EXPORT_CODE double CONVENTION K2F(double T_K);

EXPORT_CODE long CONVENTION get_param_index(const char *param);

EXPORT_CODE long CONVENTION get_input_pair_index(const char *pair);

EXPORT_CODE long CONVENTION redirect_stdout(const char *file);

EXPORT_CODE int CONVENTION get_debug_level();

EXPORT_CODE void CONVENTION set_debug_level(int level);

EXPORT_CODE double CONVENTION saturation_ancillary(const char *fluid_name, const char *output,
                                                   int Q, const char *input, double value);

EXPORT_CODE double CONVENTION HAPropsSI(const char *Output, const char *Name1, double Prop1,
                                        const char *Name2, double Prop2, const char *Name3,
                                        double Prop3);

EXPORT_CODE double CONVENTION cair_sat(double T);

EXPORT_CODE void CONVENTION hapropssi_(const char *Output, const char *Name1, const double *Prop1,
                                       const char *Name2, const double *Prop2, const char *Name3,
                                       const double *Prop3, double *output);

EXPORT_CODE double CONVENTION HAProps(const char *Output, const char *Name1, double Prop1,
                                      const char *Name2, double Prop2, const char *Name3,
                                      double Prop3);

EXPORT_CODE void CONVENTION haprops_(const char *Output, const char *Name1, const double *Prop1,
                                     const char *Name2, const double *Prop2, const char *Name3,
                                     const double *Prop3, double *output);

EXPORT_CODE long CONVENTION AbstractState_factory(const char *backend, const char *fluids,
                                                  long *errcode, char *message_buffer,
                                                  const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_fluid_names(const long handle, char *fluids,
                                                      long *errcode, char *message_buffer,
                                                      const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_free(const long handle, long *errcode,
                                               char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_set_fractions(const long handle, const double *fractions,
                                                        const long N, long *errcode,
                                                        char *message_buffer,
                                                        const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_get_mole_fractions(const long handle, double *fractions,
                                                             const long maxN, long *N,
                                                             long *errcode, char *message_buffer,
                                                             const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_get_mole_fractions_satState(
    const long handle, const char *saturated_state, double *fractions, const long maxN, long *N,
    long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_get_fugacity(const long handle, const long i,
                                                         long *errcode, char *message_buffer,
                                                         const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_get_fugacity_coefficient(const long handle,
                                                                     const long i, long *errcode,
                                                                     char *message_buffer,
                                                                     const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_update(const long handle, const long input_pair,
                                                 const double value1, const double value2,
                                                 long *errcode, char *message_buffer,
                                                 const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_specify_phase(const long handle, const char *phase,
                                                        long *errcode, char *message_buffer,
                                                        const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_unspecify_phase(const long handle, long *errcode,
                                                          char *message_buffer,
                                                          const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_keyed_output(const long handle, const long param,
                                                         long *errcode, char *message_buffer,
                                                         const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_first_saturation_deriv(const long handle, const long Of,
                                                                   const long Wrt, long *errcode,
                                                                   char *message_buffer,
                                                                   const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_first_partial_deriv(const long handle, const long Of,
                                                                const long Wrt, const long Constant,
                                                                long *errcode, char *message_buffer,
                                                                const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_second_two_phase_deriv(
    const long handle, const long Of1, const long Wrt1, const long Constant1, const long Wrt2,
    const long Constant2, long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_second_partial_deriv(
    const long handle, const long Of1, const long Wrt1, const long Constant1, const long Wrt2,
    const long Constant2, long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_first_two_phase_deriv_splined(
    const long handle, const long Of, const long Wrt, const long Constant, const double x_end,
    long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_first_two_phase_deriv(
    const long handle, const long Of, const long Wrt, const long Constant, long *errcode,
    char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_update_and_common_out(
    const long handle, const long input_pair, const double *value1, const double *value2,
    const long length, double *T, double *p, double *rhomolar, double *hmolar, double *smolar,
    long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_update_and_1_out(const long handle, const long input_pair,
                                                           const double *value1,
                                                           const double *value2, const long length,
                                                           const long output, double *out,
                                                           long *errcode, char *message_buffer,
                                                           const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_update_and_5_out(
    const long handle, const long input_pair, const double *value1, const double *value2,
    const long length, long *outputs, double *out1, double *out2, double *out3, double *out4,
    double *out5, long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_set_binary_interaction_double(
    const long handle, const long i, const long j, const char *parameter, const double value,
    long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_set_cubic_alpha_C(const long handle, const long i,
                                                            const char *parameter, const double c1,
                                                            const double c2, const double c3,
                                                            long *errcode, char *message_buffer,
                                                            const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_set_fluid_parameter_double(
    const long handle, const long i, const char *parameter, const double value, long *errcode,
    char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_build_phase_envelope(const long handle, const char *level,
                                                               long *errcode, char *message_buffer,
                                                               const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_get_phase_envelope_data(
    const long handle, const long length, double *T, double *p, double *rhomolar_vap,
    double *rhomolar_liq, double *x, double *y, long *errcode, char *message_buffer,
    const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_get_phase_envelope_data_checkedMemory(
    const long handle, const long length, const long maxComponents, double *T, double *p,
    double *rhomolar_vap, double *rhomolar_liq, double *x, double *y, long *actual_length,
    long *actual_components, long *errcode, char *message_buffer, const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_build_spinodal(const long handle, long *errcode,
                                                         char *message_buffer,
                                                         const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_get_spinodal_data(const long handle, const long length,
                                                            double *tau, double *delta, double *M1,
                                                            long *errcode, char *message_buffer,
                                                            const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_all_critical_points(const long handle, const long length,
                                                              double *T, double *p,
                                                              double *rhomolar, long *stable,
                                                              long *errcode, char *message_buffer,
                                                              const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_keyed_output_satState(const long handle,
                                                                  const char *saturated_state,
                                                                  const long param, long *errcode,
                                                                  char *message_buffer,
                                                                  const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_backend_name(const long handle, char *backend,
                                                       long *errcode, char *message_buffer,
                                                       const long buffer_length);

EXPORT_CODE void CONVENTION AbstractState_fluid_param_string(const long handle, const char *param,
                                                             char *return_buffer,
                                                             const long return_buffer_length,
                                                             long *errcode, char *message_buffer,
                                                             const long buffer_length);

EXPORT_CODE int CONVENTION AbstractState_phase(const long handle, long *errcode,
                                               char *message_buffer, const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_saturated_liquid_keyed_output(const long handle,
                                                                          const long param,
                                                                          long *errcode,
                                                                          char *message_buffer,
                                                                          const long buffer_length);

EXPORT_CODE double CONVENTION AbstractState_saturated_vapor_keyed_output(const long handle,
                                                                         const long param,
                                                                         long *errcode,
                                                                         char *message_buffer,
                                                                         const long buffer_length);

EXPORT_CODE void CONVENTION add_fluids_as_JSON(const char *backend, const char *fluidstring,
                                               long *errcode, char *message_buffer,
                                               const long buffer_length);

EXPORT_CODE int CONVENTION C_is_valid_fluid_string(const char *fluidName);

EXPORT_CODE int CONVENTION C_extract_backend(const char *fluid_string, char *backend,
                                             const long backend_length, char *fluid,
                                             const long fluid_length);

#endif
