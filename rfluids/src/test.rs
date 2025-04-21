macro_rules! assert_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::assert_relative_eq!($lhs, $rhs, max_relative = 1e-6);
    };
}

pub(crate) use assert_relative_eq;

macro_rules! test_input {
    ($type:ty: $name:ident, key: $key:expr) => {
        paste::paste! {
            #[test]
            fn [<$name _returns_expected_key_and_value>]() {
                let sut = $type::$name(42.0);
                assert_eq!(sut.key, $key);
                assert_eq!(sut.value, 42.0);
            }
        }
    };
    ($type:ty: $name:ident, key: $key:expr, reciprocal) => {
        paste::paste! {
            #[test]
            fn [<$name _returns_expected_key_and_value>]() {
                let sut = $type::$name(42.0);
                assert_eq!(sut.key, $key);
                assert_eq!(sut.value, 1.0 / 42.0);
            }
        }
    };
}

pub(crate) use test_input;

macro_rules! test_output {
    ($type:ty: $name:ident, $ok:ident: $ok_value:expr $(, $err:ident: Err)*) => {
        paste::paste! {
            #[rstest::rstest]
            fn [<$name _returns_expected_value>](
                mut $ok: $type,
                $(mut $err: $type,)*
            ) {
                assert!($ok.$name().is_ok());
                $crate::test::assert_relative_eq!($ok.$name().unwrap(), $ok_value);
                $(assert!($err.$name().is_err());)*
            }
        }
    };
    ($type:ty: $name:ident, $ok:ident: $ok_value:expr, always_ok) => {
        paste::paste! {
            #[rstest::rstest]
            fn [<$name _returns_expected_value>](mut $ok: $type) {
                $crate::test::assert_relative_eq!($ok.$name(), $ok_value);
            }
        }
    };
}

pub(crate) use test_output;
