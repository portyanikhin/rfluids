use rfluids::config::{self, Config};
use rstest::*;

#[fixture]
fn new_cfg() -> Config {
    Config::builder()
        .assume_critical_point_is_stable(true)
        .critical_within_1uk(false)
        .dont_check_prop_limits(true)
        .enable_critical_splines(false)
        .enable_superancillaries(false)
        .henrys_law_to_generate_vle_guesses(true)
        .normalize_gas_constants(false)
        .overwrite_binary_interaction(true)
        .overwrite_departure_fn(true)
        .overwrite_substances(true)
        .phase_envelope_start_pressure_pa(42.0)
        .ru_codata(8.314)
        .spinodal_min_delta(1.0)
        .use_guesses_in_props_si(true)
        .alt_refprop_path("path/to/refprop")
        .alt_refprop_lib_path("path/to/refprop/lib")
        .alt_refprop_hmx_bnc_path("path/to/refprop/hmx.bnc")
        .refprop_dont_estimate_interaction_params(true)
        .refprop_ignore_error_estimated_interaction_params(true)
        .refprop_use_gerg(true)
        .refprop_use_peng_robinson(true)
        .alt_tables_path("path/to/tables")
        .float_punctuation(',')
        .list_punctuation(';')
        .max_table_dir_size_in_gb(42.0)
        .save_raw_tables(true)
        .vtpr_always_reload_lib(true)
        .vtpr_unifac_path("path/to/unifac/json")
        .build()
}

#[rstest]
fn config_management(new_cfg: Config) {
    // When
    let cfg = config::read();

    // Then
    assert_eq!(cfg, Config::default());

    // When
    config::update(new_cfg.clone());
    let cfg = config::read();

    // Then
    assert_eq!(cfg, new_cfg);

    // When
    config::reset();
    let cfg = config::read();

    // Then
    assert_eq!(cfg, Config::default());
}
