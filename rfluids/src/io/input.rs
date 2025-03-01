/// Keyed input.
pub trait Input<T: Copy> {
    /// Specified key.
    fn key(&self) -> T;

    /// Specified value _(in SI units)_.
    fn si_value(&self) -> f64;
}

macro_rules! impl_input {
    ($type:ty, $key_type:ty) => {
        impl $crate::io::Input<$key_type> for $type {
            fn key(&self) -> $key_type {
                self.0
            }

            fn si_value(&self) -> f64 {
                self.1
            }
        }
    };
}

pub(crate) use impl_input;

macro_rules! define_input {
    (
        $mod:ident,
        $name:ident,
        $key_type:ident,
        $key:ident,
        $type:ty,
        $description:literal,
        $units:literal
    ) => {
        paste::paste! {
            #[doc = $description " _(key: [`" $key "`](" $key_type "::" $key "), "]
            #[doc = "SI units: " $units ")_."]
            #[doc = "\n\n# See also\n\n"]
            #[doc = "- [`" $mod "::" $name "!`](crate::io::" $mod "::" $name ") macro\n"]
            #[doc = "- [`" $name "_si`](Self::" $name "_si)"]
            #[must_use]
            pub fn $name(value: $type) -> Self {
                Self::[<$name _si>](value.value)
            }

            #[doc = $description " _(key: [`" $key "`](" $key_type "::" $key "))_ "]
            #[doc = "in SI units _(" $units ")_."]
            #[doc = "\n\n# See also\n\n"]
            #[doc = "- [`" $mod "::" $name "!`](crate::io::" $mod "::" $name ") macro\n"]
            #[doc = "- [`" $name "`](Self::" $name ")"]
            #[must_use]
            pub fn [<$name _si>](value: f64) -> Self {
                Self($key_type::$key, value)
            }
        }
    };
}

pub(crate) use define_input;

macro_rules! define_input_macro {
    (
        $mod:ident,
        $type:ident,
        $name:ident,
        $value_type:ident,
        $prelude_mod:ident,
        $example_unit_mod:ident,
        $example_unit:ident
    ) => {
        paste::paste! {
            #[doc = "Shortcut for [`" $type "::" $name "`] and "]
            #[doc = "[`" $type "::" $name "_si`]."]
            #[doc = "\n\n# Args\n\n"]
            #[doc = "The first argument is the value of the input parameter _([`f64`])_, "]
            #[doc = "and the second argument _(optional)_ is the unit of measure _(e.g., "]
            #[doc = "[`" $example_unit "`](uom::si::" $example_unit_mod "::" $example_unit "))_."]
            #[doc = "\n\nIf the unit of measure is not provided, "]
            #[doc = "the value is assumed to be in SI units."]
            #[doc = "\n\n# Examples\n\n"]
            #[doc = "```\n"]
            #[doc = "use rfluids::prelude::" $prelude_mod "::*;\n"]
            #[doc = "use uom::si::" $example_unit_mod "::" $example_unit ";\n\n"]
            #[doc = "assert_eq!(\n"]
            #[doc = "    " $mod "::" $name "!(42.0, " $example_unit "),\n"]
            #[doc = "    " $type "::" $name "(" $value_type "::new::<" $example_unit ">(42.0))\n"]
            #[doc = ");\n"]
            #[doc = "```\n\n"]
            #[doc = "```\n"]
            #[doc = "use rfluids::prelude::" $prelude_mod "::*;\n\n"]
            #[doc = "assert_eq!(\n"]
            #[doc = "    " $mod "::" $name "!(42.0),\n"]
            #[doc = "    " $type "::" $name "_si(42.0)\n"]
            #[doc = ");\n"]
            #[doc = "```"]
            #[macro_export]
            macro_rules! [<$mod _ $name>] {
                ($value:expr, $unit:ty) => {
                    $crate::io::$mod::$type::$name(uom::si::f64::$value_type::new::<$unit>(
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
