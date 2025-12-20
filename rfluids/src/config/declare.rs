macro_rules! declare_config {
    ($($(#[$meta:meta])* $field:ident : $ty:ty $(= $default:expr)? => $key:ident),* $(,)?) => {
        /// Crate configuration state.
        ///
        /// This configuration mirrors the configuration options available in `CoolProp`.
        /// Each field corresponds directly to a `CoolProp` configuration parameter and
        /// is synchronized with the underlying `CoolProp` library when
        /// [`config::update`](crate::config::update) is called.
        ///
        /// # Thread Safety
        ///
        /// The configuration is stored in a global state protected by an
        /// [`RwLock`](std::sync::RwLock). Use [`config::read`](crate::config::read) to get
        /// the current configuration and [`config::update`](crate::config::update) to modify it.
        #[derive(Clone, Debug, PartialEq, bon::Builder)]
        #[builder(on(PathBuf, into))]
        #[non_exhaustive]
        pub struct Config {
            $(
                $(#[$meta])*
                $(#[builder(default = $default)])?
                pub $field: $ty,
            )*
        }

        impl Config {
            fn update(&mut self, new: Self) {
                $(if self.$field != new.$field {
                    let key = crate::io::ConfigKey::$key;
                    crate::native::CoolProp::set_config(key, &new.$field).unwrap();
                    self.$field = new.$field;
                })*
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
