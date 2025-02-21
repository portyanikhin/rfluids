macro_rules! assert_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::assert_relative_eq!($lhs, $rhs, max_relative = 1e-6);
    };
}

pub(crate) use assert_relative_eq;

macro_rules! test_input {
    ($name:ident, $type:ident, $key:expr, $value_type:ident, $unit:ty) => {
        paste::paste! {
            #[test]
            fn [<$name _returns_expected_key_and_si_value>]() {
                let sut = $type::$name($value_type::new::<$unit>(1.0));
                assert_eq!(sut.key(), $key);
                assert_eq!(sut.si_value(), 1.0);
                assert_eq!(sut, $type::[<$name _si>](1.0));
                assert_eq!(sut, $name!(1.0, $unit));
                assert_eq!(sut, $name!(1.0));
            }
        }
    };
}

pub(crate) use test_input;

pub(crate) mod fluid {
    macro_rules! test_output {
        ($fluid_type:ty, $name:ident, $ok_fluid:ident, $ok_value:expr $(, $err_fluid:ident)*) => {
            paste::paste! {
                #[rstest::rstest]
                fn [<$name _returns_expected_value>](
                    mut $ok_fluid: $fluid_type,
                    $(mut $err_fluid: $fluid_type,)*
                ) {
                    assert!($ok_fluid.$name().is_ok());
                    $crate::test::assert_relative_eq!($ok_fluid.$name().unwrap().value, $ok_value);
                    $(assert!($err_fluid.$name().is_err());)*
                }
            }
        };
        ($fluid_type:ty, f64, $name:ident, $ok_fluid:ident, $ok_value:expr $(, $err_fluid:ident)*) => {
            paste::paste! {
                #[rstest::rstest]
                fn [<$name _returns_expected_value>](
                    mut $ok_fluid: $fluid_type,
                    $(mut $err_fluid: $fluid_type,)*
                ) {
                    assert!($ok_fluid.$name().is_ok());
                    $crate::test::assert_relative_eq!($ok_fluid.$name().unwrap(), $ok_value);
                    $(assert!($err_fluid.$name().is_err());)*
                }
            }
        };
        ($fluid_type:ty, always_ok, $name:ident, $fluid:ident, $value:expr) => {
            paste::paste! {
                #[rstest::rstest]
                fn [<$name _returns_expected_value>](mut $fluid: $fluid_type) {
                    $crate::test::assert_relative_eq!($fluid.$name().value, $value);
                }
            }
        };
    }

    pub(crate) use test_output;
}
