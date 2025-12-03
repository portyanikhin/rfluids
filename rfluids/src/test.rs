pub(crate) trait SutFactory<P> {
    type Sut;

    fn sut(&self, payload: P) -> Self::Sut;
}

macro_rules! assert_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::assert_relative_eq!($lhs, $rhs, max_relative = 1e-6);
    };
}

pub(crate) use assert_relative_eq;

macro_rules! test_input {
    ($name:ident,key: $type:ident:: $key:ident) => {
        #[test]
        fn $name() {
            // Given
            let sut = $crate::io::Input::<$type>::$name(42.0);

            // When
            let (key, value) = (sut.key, sut.value);

            // Then
            assert_eq!(key, $type::$key);
            assert_eq!(value, 42.0);
        }
    };
    ($name:ident,key: $type:ident:: $key:ident,reciprocal) => {
        #[test]
        fn $name() {
            // Given
            let sut = $crate::io::Input::<$type>::$name(42.0);

            // When
            let (key, value) = (sut.key, sut.value);

            // Then
            assert_eq!(key, $type::$key);
            assert_eq!(value, 1.0 / 42.0);
        }
    };
}

pub(crate) use test_input;

macro_rules! test_output {
    ($name:ident, $ok:ident: $ok_value:expr $(, $err:ident: Err)*) => {
        paste::paste! {
            #[rstest::rstest]
            fn [<$name _ $ok>](ctx: Context) {
                // Given
                let Context { $ok, .. } = ctx;
                let mut sut = ctx.sut($ok);

                // When
                let res = sut.$name();

                // Then
                assert!(res.is_ok());
                $crate::test::assert_relative_eq!(res.unwrap(), $ok_value);
            }$(

            #[rstest::rstest]
            fn [<$name _ $err>](ctx: Context) {
                // Given
                let Context { $err, .. } = ctx;
                let mut sut = ctx.sut($err);

                // When
                let res = sut.$name();

                // Then
                assert!(res.is_err());
            })*
        }
    };
    ($name:ident, $ok:ident: $ok_value:expr, always_ok) => {
        #[rstest::rstest]
        fn $name(ctx: Context) {
            // Given
            let Context { $ok, .. } = ctx;
            let mut sut = ctx.sut($ok);

            // When
            let res = sut.$name();

            // Then
            $crate::test::assert_relative_eq!(res, $ok_value);
        }
    };
}

pub(crate) use test_output;
