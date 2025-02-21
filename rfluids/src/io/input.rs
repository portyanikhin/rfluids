/// Keyed input.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Input<T: Copy>(pub(crate) T, pub(crate) f64);

impl<T: Copy> Input<T> {
    /// Specified key.
    pub fn key(&self) -> T {
        self.0
    }

    /// Specified value _(in SI units)_.
    pub fn si_value(&self) -> f64 {
        self.1
    }
}

macro_rules! input_doc {
    ($key_type:ident, $key:ident, $description:literal, $units:literal) => {
        concat!(
            $description,
            " _(key: [`",
            stringify!($key),
            "`](",
            stringify!($key_type),
            "::",
            stringify!($key),
            "), SI units: ",
            $units,
            ")_."
        )
    };
    (si, $key_type:ident, $key:ident, $description:literal, $units:literal) => {
        concat!(
            $description,
            " _(key: [`",
            stringify!($key),
            "`](",
            stringify!($key_type),
            "::",
            stringify!($key),
            "))_ in SI units _(",
            $units,
            ")_."
        )
    };
}

pub(crate) use input_doc;

macro_rules! define_input {
    ($name:ident, $key_type:ident, $key:ident, $type:ty, $description:literal, $units:literal) => {
        #[doc = $crate::io::input_doc!($key_type, $key, $description, $units)]
        #[must_use]
        pub fn $name(value: $type) -> Self {
            Self($key_type::$key, value.value)
        }

        paste::paste! {
            #[doc = $crate::io::input_doc!(si, $key_type, $key, $description, $units)]
            #[must_use]
            pub fn [<$name _si>](value: f64) -> Self {
                Self($key_type::$key, value)
            }
        }
    };
}

pub(crate) use define_input;

macro_rules! input_macro_doc {
    (
        $mod:ident,
        $type:ident,
        $name:ident,
        $value_type:ident,
        $prelude_path:path,
        $example_unit_path:path,
        $example_unit:ty
    ) => {
        concat!(
            "Shortcut for [`",
            stringify!($type),
            "::",
            stringify!($name),
            "`] and [`",
            stringify!($type),
            "::",
            stringify!($name),
            "_si`].",
            "\n\n# Args\n\n",
            "The first argument is the value of the input parameter _([`f64`])_, ",
            "and the second argument _(optional)_ is the unit of measure _(e.g., [`",
            stringify!($example_unit),
            "`](",
            stringify!($example_unit_path),
            "::",
            stringify!($example_unit),
            "))_.\n\nIf the unit of measure is not provided, ",
            "the value is assumed to be in SI units.",
            "\n\n# Examples\n\n",
            "```\n",
            "use ",
            stringify!($prelude_path),
            "::*;\n",
            "use rfluids::",
            stringify!($example_unit_path),
            "::",
            stringify!($example_unit),
            ";\n\n",
            "assert_eq!(\n    ",
            stringify!($mod),
            "::",
            stringify!($name),
            "!(42.0, ",
            stringify!($example_unit),
            "),\n    ",
            stringify!($type),
            "::",
            stringify!($name),
            "(",
            stringify!($value_type),
            "::new::<",
            stringify!($example_unit),
            ">(42.0))\n",
            ");\n",
            "```\n\n",
            "```\n",
            "use ",
            stringify!($prelude_path),
            "::*;\n\n",
            "assert_eq!(\n    ",
            stringify!($mod),
            "::",
            stringify!($name),
            "!(42.0),\n    ",
            stringify!($type),
            "::",
            stringify!($name),
            "_si(42.0)\n",
            ");\n",
        )
    };
}

pub(crate) use input_macro_doc;

macro_rules! define_input_macro {
    (
        $mod:ident,
        $type:ident,
        $name:ident,
        $value_type:ident,
        $prelude_path:path,
        $example_unit_path:path,
        $example_unit:ty
    ) => {
        paste::paste! {
            #[doc = $crate::io::input_macro_doc!(
                $mod, $type, $name, $value_type, $prelude_path, $example_unit_path, $example_unit
            )]
            #[macro_export]
            macro_rules! [<$mod _ $name>] {
                ($value:expr, $unit:ty) => {
                    $crate::io::$mod::$type::$name($crate::uom::si::f64::$value_type::new::<$unit>(
                        $value,
                    ))
                };
                ($value:expr) => {
                        $crate::io::$mod::$type::[<$name _si>]($value)
                };
            }

            pub use [<$mod _ $name>] as $name;
        }
    };
}

pub(crate) use define_input_macro;
