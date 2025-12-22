macro_rules! declare_config {
    (
        @custom {
            $(
                $(#[$custom_meta:meta])*
                $custom_vis:vis $custom_field:ident : $custom_ty:ty $(= $custom_default:expr)? =>
                    (get: $custom_getter:expr, set: $custom_setter:expr)
            ),* $(,)?
        }
        $(
            $(#[$meta:meta])*
            $vis:vis $field:ident : $ty:ty $(= $default:expr)? => $key:ident
        ),* $(,)?
    ) => {
        /// Crate configuration state.
        ///
        /// This configuration mirrors the configuration options available in `CoolProp`.
        /// Each field corresponds directly to a `CoolProp` configuration parameter and
        /// is synchronized with the underlying `CoolProp` library when
        /// [`config::update`](crate::config::update) is called.
        ///
        /// # [`serde`](https://crates.io/crates/serde) Support
        ///
        /// Enable the `serde` feature for serialization and deserialization support.
        /// See the [module documentation](crate::config#serde-support) for details and examples.
        ///
        /// # Thread Safety
        ///
        /// The configuration is stored in a global state protected by an
        /// [`RwLock`](std::sync::RwLock). Use [`config::read`](crate::config::read) to get
        /// the current configuration and [`config::update`](crate::config::update) to modify it.
        #[derive(Clone, Debug, PartialEq, bon::Builder)]
        #[builder(on(PathBuf, into))]
        #[non_exhaustive]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(default))]
        pub struct Config {
            $(
                $(#[$custom_meta])*
                $(#[builder(default = $custom_default)])?
                $custom_vis $custom_field: $custom_ty,
            )*
            $(
                $(#[$meta])*
                $(#[builder(default = $default)])?
                $vis $field: $ty,
            )*
        }

        impl Config {
            fn update(&mut self, new: Self) {
                $(
                    if self.$custom_field != new.$custom_field {
                        $custom_setter(new.$custom_field);
                        self.$custom_field = $custom_getter();
                    }
                )*
                $(
                    if self.$field != new.$field {
                        let key = crate::io::ConfigKey::$key;
                        crate::native::CoolProp::set_config(key, &new.$field).unwrap();
                        self.$field = new.$field;
                    }
                )*
            }
        }

        impl Default for Config {
            fn default() -> Self {
                Config::builder().build()
            }
        }
    };
}

pub(super) use declare_config;
