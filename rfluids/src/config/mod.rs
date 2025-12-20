//! Library configuration.

#![allow(clippy::struct_excessive_bools)]

mod declare;

use std::{
    path::PathBuf,
    sync::{LazyLock, RwLock},
};

use declare::declare_config;

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::default()));

declare_config! {
    /// If `true`, evaluation of the stability of critical point will be skipped and point will be
    /// assumed to be stable.
    ///
    /// **Default:** `false`
    assume_critical_point_is_stable: bool = false => AssumeCriticalPointIsStable,

    /// if `true`, any temperature within `1 uK` of the critical temperature will be considered to
    /// be **AT** the critical point.
    ///
    /// **Default:** `true`
    critical_within_1uk: bool = true => CriticalWithin1Uk,

    /// If `true`, when possible, `CoolProp` will skip checking whether values are inside the
    /// property limits.
    ///
    /// **Default:** `false`
    dont_check_prop_limits: bool = false => DontCheckPropLimits,

    /// If `true`, the critical splines will be used in the near-vicinity of the critical point.
    ///
    /// **Default:** `true`
    enable_critical_splines: bool = true => EnableCriticalSplines,

    /// if `true`, the superancillary functions will be used for VLE of pure fluids.
    ///
    /// **Default:** `true`
    enable_superancillaries: bool = true => EnableSuperancillaries,

    /// If `true`, when doing water-based mixture dewpoint calculations, use Henry’s Law to
    /// generate guesses for liquid-phase composition.
    ///
    /// **Default:** `false`
    henrys_law_to_generate_vle_guesses: bool = false => HenrysLawToGenerateVleGuesses,

    /// If `true`, for mixtures, the molar gas constant _(R)_ will be set to the CODATA value.
    ///
    /// **Default:** `true`
    normalize_gas_constants: bool = true => NormalizeGasConstants,

    /// If `true`, and a pair of binary interaction pairs to be added is already there, rather than
    /// not adding the binary interaction pair _(and probably throwing an exception)_, overwrite
    /// it.
    ///
    /// **Default:** `false`
    overwrite_binary_interaction: bool = false => OverwriteBinaryInteraction,

    /// if `true`, and a departure function to be added is already there, rather than not adding
    /// the departure function _(and probably throwing an exception)_, overwrite it.
    ///
    /// **Default:** `false`
    overwrite_departure_fn: bool = false => OverwriteDepartureFn,

    /// If `true`, and a substance is added to the substances library that is already there, rather
    /// than not adding the substance _(and probably throwing an exception)_, overwrite it.
    ///
    /// **Default:** `false`
    overwrite_substances: bool = false => OverwriteSubstances,

    /// Starting pressure in `Pa` for phase envelope construction.
    ///
    /// **Default:** `100.0`
    phase_envelope_start_pressure_pa: f64 = 100.0 => PhaseEnvelopeStartPressurePa,

    /// The value for the ideal gas constant in `J/mol/K` according to CODATA 2022. This value is
    /// used to harmonize all the ideal gas constants. This is especially important in the critical
    /// region.
    ///
    /// **Default:** `8.314_462_618_153_24`
    ru_codata: f64 = 8.314_462_618_153_24 => RUCodata,

    /// The minimal delta to be used in tracing out the spinodal; make sure that the EOS has a
    /// spinodal at this value of `delta=rho/rho_r`.
    ///
    /// **Default:** `0.5`
    spinodal_min_delta: f64 = 0.5 => SpinodalMinDelta,

    /// If `true`, calls to the vectorized versions of `PropsSI` use the previous state as guess
    /// value while looping over the input vectors, only makes sense when working with a single
    /// fluid and with points that are not too far from each other.
    ///
    /// **Default:** `false`
    use_guesses_in_props_si: bool = false => UseGuessesInPropsSi,

    /// An alternative path to be provided to the directory that contains `REFPROP`’s fluids and
    /// mixtures directories. If provided, the `SETPATH` function will be called with this
    /// directory prior to calling any `REFPROP` functions.
    ///
    /// **Default:** `None`
    alt_refprop_path: Option<PathBuf> => AltRefpropPath,

    /// An alternative path to the shared library file. If provided, it will be used to load
    /// `REFPROP`.
    ///
    /// **Default:** `None`
    alt_refprop_lib_path: Option<PathBuf> => AltRefpropLibPath,

    /// An alternative path to the `HMX.BNC` file. If provided, it will be passed into `REFPROP`’s
    /// `SETUP` or `SETMIX` routines.
    ///
    /// **Default:** `None`
    alt_refprop_hmx_bnc_path: Option<PathBuf> => AltRefpropHmxBncPath,

    /// If `true`, if the binary interaction parameters in `REFPROP` are estimated, throw an error
    /// rather than silently continuing.
    ///
    /// **Default:** `false`
    refprop_dont_estimate_interaction_params: bool = false => RefpropDontEstimateInteractionParams,

    /// If `true`, if the binary interaction parameters in `REFPROP` are unable to be estimated,
    /// silently continue rather than failing.
    ///
    /// **Default:** `false`
    refprop_ignore_error_estimated_interaction_params: bool = false =>
        RefpropIgnoreErrorEstimatedInteractionParams,

    /// If `true`, rather than using the highly-accurate pure fluid equations of state, use the
    /// pure-fluid EOS from `GERG-2008`.
    ///
    /// **Default:** `false`
    refprop_use_gerg: bool = false => RefpropUseGerg,

    /// If `true`, rather than using the highly-accurate pure fluid equations of state, use the
    /// Peng-Robinson EOS.
    ///
    /// **Default:** `false`
    refprop_use_peng_robinson: bool = false => RefpropUsePengRobinson,

    /// If provided, this path will be the root directory for the tabular data. Otherwise,
    /// `${HOME}/.CoolProp/Tables` is used.
    ///
    /// **Default:** `None`
    alt_tables_path: Option<PathBuf> => AltTablesPath,

    /// The first character of this string will be used as the separator between the number
    /// fraction.
    ///
    /// **Default:** `'.'`
    float_punctuation: char = '.' => FloatPunctuation,

    /// The delimiter to be used when converting a list of strings to a string.
    ///
    /// **Default:** `','`
    list_punctuation: char = ',' => ListPunctuation,

    /// The maximum allowed size of the directory that is used to store tabular data.
    ///
    /// **Default:** `1.0`
    max_table_dir_size_in_gb: f64 = 1.0 => MaxTableDirSizeInGb,

    /// If `true`, the raw, uncompressed tables will also be written to file.
    ///
    /// **Default:** `false`
    save_raw_tables: bool = false => SaveRawTables,

    /// If `true`, the library will always be reloaded, no matter what is currently loaded.
    ///
    /// **Default:** `false`
    vtpr_always_reload_lib: bool = false => VtPrAlwaysReloadLib,

    /// The path to the directory containing the UNIFAC JSON files.
    ///
    /// **Default:** `None`
    vtpr_unifac_path: Option<PathBuf> => VtPrUnifacPath,
}

/// Returns a clone of the current library configuration.
///
/// This function acquires a read lock on the global configuration state and returns
/// a clone of the current [`Config`].
///
/// # Panics
///
/// Panics if the internal read lock is poisoned. This should only occur if another thread
/// panicked while holding the write lock.
///
/// # Examples
///
/// ```no_run
/// use rfluids::config;
///
/// let cfg = config::read();
/// println!("{cfg:?}");
/// ```
pub fn read() -> Config {
    CONFIG.read().unwrap().clone()
}

/// Updates the library configuration with new values.
///
/// To create a new configuration, use [`Config::builder`] to build a configuration
/// with only the fields you want to change.
///
/// This function compares each field of the new configuration with the current one.
/// For any field that has changed, it synchronizes the value with the underlying `CoolProp`
/// library before updating the global state.
///
/// # Panics
///
/// Panics if `CoolProp` rejects a configuration value. This should never happen in practice,
/// as the implementation uses only valid configuration keys with correctly typed values.
/// `CoolProp` validates only the type correctness for each existing configuration key.
///
/// Also panics if the internal write lock is poisoned. This should only occur if another thread
/// panicked while holding the write lock.
///
/// # Examples
///
/// ```no_run
/// use rfluids::config::{self, Config};
///
/// // Update specific configuration options
/// let new_cfg = Config::builder()
///     .enable_critical_splines(false)
///     .enable_superancillaries(false)
///     .dont_check_prop_limits(true)
///     .build();
///
/// config::update(new_cfg);
/// ```
pub fn update(new: Config) {
    let mut cfg = CONFIG.write().unwrap();
    cfg.update(new);
}

/// Resets the library configuration to its default values.
///
/// # Panics
///
/// Panics if the internal write lock is poisoned. This should only occur if another thread
/// panicked while holding the write lock.
pub fn reset() {
    update(Config::default());
}
