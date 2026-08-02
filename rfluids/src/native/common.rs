use core::ffi::{c_char, c_int, c_long};
use std::{cell::Cell, ffi::CString, marker::PhantomData};

use coolprop_sys::ExclusiveAccess;

use super::{CoolPropError, Result};
use crate::io::GlobalParam;

/// Marker to make structs `!Sync` while preserving `Send`.
pub(crate) type PhantomUnsync = PhantomData<Cell<()>>;

const ERROR_BUFFER_CAPACITY: usize = 500;

#[derive(Debug)]
pub(crate) struct ErrorBuffer {
    err_code: c_long,
    err_message: [u8; ERROR_BUFFER_CAPACITY],
}

impl ErrorBuffer {
    #[must_use]
    pub fn as_mut_parts(&mut self) -> (*mut c_long, *mut c_char, c_long) {
        let capacity = c_long::try_from(self.err_message.len())
            .expect("error buffer capacity must fit into `c_long`");
        (&raw mut self.err_code, self.err_message.as_mut_ptr().cast(), capacity)
    }

    pub fn into_result(self) -> Result<()> {
        if self.err_code == 0 {
            return Ok(());
        }
        let err_code = self.err_code;
        let err_message = string_from_bytes(&self.err_message);
        if err_message.trim().is_empty() {
            Err(CoolPropError::Native(format!(
                "CoolProp native call failed with error code {err_code} and no error message"
            )))
        } else {
            Err(CoolPropError::Native(err_message))
        }
    }

    #[cfg(test)]
    #[must_use]
    pub fn code(&self) -> c_long {
        self.err_code
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self { err_code: 0, err_message: [0; ERROR_BUFFER_CAPACITY] }
    }
}

#[derive(Debug)]
pub(crate) struct StringBuffer {
    capacity: c_int,
    buffer: Box<[u8]>,
}

impl StringBuffer {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let abi_capacity =
            c_int::try_from(capacity).expect("string buffer capacity exceeds `c_int::MAX`");
        let storage_len = capacity.max(1);
        Self { capacity: abi_capacity, buffer: vec![0; storage_len].into_boxed_slice() }
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.buffer.as_mut_ptr().cast()
    }

    #[must_use]
    pub fn capacity(&self) -> c_int {
        self.capacity
    }
}

impl Default for StringBuffer {
    fn default() -> Self {
        Self::with_capacity(ERROR_BUFFER_CAPACITY)
    }
}

impl From<StringBuffer> for String {
    fn from(value: StringBuffer) -> Self {
        let capacity = usize::try_from(value.capacity)
            .expect("string buffer capacity is checked at construction");
        string_from_bytes(&value.buffer[..capacity])
    }
}

fn string_from_bytes(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

impl From<StringBuffer> for Option<CoolPropError> {
    fn from(value: StringBuffer) -> Self {
        let message: String = value.into();
        if message.trim().is_empty() { None } else { Some(CoolPropError::Native(message)) }
    }
}

pub(crate) fn c_string(arg: &'static str, value: impl Into<Vec<u8>>) -> Result<CString> {
    CString::new(value).map_err(|err| CoolPropError::InteriorNul { arg, pos: err.nul_position() })
}

pub(crate) fn c_string_trimmed(arg: &'static str, value: impl AsRef<str>) -> Result<CString> {
    c_string(arg, value.as_ref().trim())
}

pub(crate) fn factory_requires_exclusive(backend: &str) -> bool {
    !["HEOS", "INCOMP", "IF97", "SRK", "PR", "PCSAFT"]
        .into_iter()
        .any(|known| backend.trim().eq_ignore_ascii_case(known))
}

pub(crate) fn state_requires_exclusive(backend: &str) -> bool {
    let backend = backend.trim();
    if backend.eq_ignore_ascii_case("VTPR") {
        return false;
    }
    if let Some(source) = svd_sbtl_source(backend) {
        return !["HEOS", "IF97"].into_iter().any(|known| source.eq_ignore_ascii_case(known));
    }
    factory_requires_exclusive(backend)
}

fn svd_sbtl_source(backend: &str) -> Option<&str> {
    let (method, source) = backend.split_once('&')?;
    method.trim().eq_ignore_ascii_case("SVDSBTL").then_some(source.trim())
}

pub(crate) fn get_error(coolprop: &ExclusiveAccess<'_>) -> Option<CoolPropError> {
    let mut message = StringBuffer::default();
    let param = CString::new(GlobalParam::PendingError.as_ref()).unwrap();
    let _unused = unsafe {
        coolprop.get_global_param_string(param.as_ptr(), message.as_mut_ptr(), message.capacity())
    };
    message.into()
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    mod error_buffer {
        use super::*;

        fn write_err_message(err_buffer: &mut ErrorBuffer, value: &str) {
            let value = CString::new(value).unwrap();
            let bytes = value.as_bytes_with_nul();
            let (_, err_message, _) = err_buffer.as_mut_parts();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr().cast::<c_char>(),
                    err_message,
                    bytes.len(),
                );
            }
        }

        fn set_code(err_buffer: &mut ErrorBuffer, value: c_long) {
            let (err_code, ..) = err_buffer.as_mut_parts();
            unsafe {
                *err_code = value;
            }
        }

        #[test]
        fn default() {
            // When
            let mut sut = ErrorBuffer::default();
            let (_, _, capacity) = sut.as_mut_parts();

            // Then
            assert_eq!(sut.code(), 0);
            assert_eq!(capacity, c_long::try_from(ERROR_BUFFER_CAPACITY).unwrap());
        }

        #[test]
        fn as_mut_parts() {
            // Given
            let mut sut = ErrorBuffer::default();

            // When
            let (err_code, err_message, capacity) = sut.as_mut_parts();
            unsafe {
                *err_code = 42;
                *err_message = b'E' as c_char;
            }
            let res = sut.into_result().unwrap_err();

            // Then
            assert_eq!(capacity, c_long::try_from(ERROR_BUFFER_CAPACITY).unwrap());
            assert_eq!(res, CoolPropError::Native("E".into()));
        }

        #[test]
        fn into_result_success_ignores_message() {
            // Given
            let mut sut = ErrorBuffer::default();
            write_err_message(&mut sut, "stale error");

            // When
            let res = sut.into_result();

            // Then
            assert!(res.is_ok());
        }

        #[test]
        fn into_result_error_with_message() {
            // Given
            let mut sut = ErrorBuffer::default();
            set_code(&mut sut, 1);
            write_err_message(&mut sut, "native error");

            // When
            let res = sut.into_result().unwrap_err();

            // Then
            assert_eq!(res, CoolPropError::Native("native error".into()));
        }

        #[rstest]
        #[case(1)]
        #[case(2)]
        #[case(3)]
        #[case(42)]
        fn into_result_error_without_message(#[case] code: c_long) {
            // Given
            let mut sut = ErrorBuffer::default();
            set_code(&mut sut, code);

            // When
            let res = sut.into_result().unwrap_err();

            // Then
            assert_eq!(
                res,
                CoolPropError::Native(format!(
                    "CoolProp native call failed with error code {code} and no error message"
                ))
            );
        }
    }

    mod string_buffer {
        use super::*;

        #[rstest]
        #[case(0)]
        #[case(42)]
        fn with_capacity(#[case] capacity: usize) {
            // When
            let sut = StringBuffer::with_capacity(capacity);

            // Then
            assert_eq!(sut.capacity(), c_int::try_from(capacity).unwrap());
        }

        #[test]
        fn zero_capacity_has_non_null_storage() {
            // Given
            let mut sut = StringBuffer::with_capacity(0);

            // When
            let pointer = sut.as_mut_ptr();

            // Then
            assert_eq!(sut.capacity(), 0);
            assert!(!pointer.is_null());
        }

        #[test]
        fn default() {
            // When
            let sut = StringBuffer::default();

            // Then
            assert_eq!(sut.capacity(), c_int::try_from(ERROR_BUFFER_CAPACITY).unwrap());
        }

        #[rstest]
        #[case("")]
        #[case("something")]
        #[case(" something else ")]
        fn into_string(#[case] value: &str) {
            // Given
            let c_string = CString::new(value).unwrap();
            let c_bytes = c_string.as_bytes_with_nul();
            let mut sut = StringBuffer::with_capacity(c_bytes.len());

            // When
            unsafe {
                std::ptr::copy_nonoverlapping(
                    c_bytes.as_ptr().cast::<c_char>(),
                    sut.as_mut_ptr(),
                    c_bytes.len(),
                );
            }
            let res: String = sut.into();

            // Then
            assert_eq!(res, value);
        }

        #[test]
        fn into_string_empty() {
            // Given
            let sut = StringBuffer::with_capacity(42);

            // When
            let res: String = sut.into();

            // Then
            assert!(res.is_empty());
        }

        #[test]
        fn into_string_zero_capacity() {
            // Given
            let sut = StringBuffer::with_capacity(0);

            // When
            let res: String = sut.into();

            // Then
            assert!(res.is_empty());
        }

        #[test]
        fn into_string_lossy() {
            // Given
            let invalid_utf8: Vec<u8> = vec![
                b'H', b'e', b'l', b'l', b'o', 0xFF, 0xFE, b'W', b'o', b'r', b'l', b'd', b'!', b'\0',
            ];
            let mut sut = StringBuffer::with_capacity(invalid_utf8.len());

            // When
            unsafe {
                std::ptr::copy_nonoverlapping(
                    invalid_utf8.as_ptr().cast::<c_char>(),
                    sut.as_mut_ptr(),
                    invalid_utf8.len(),
                );
            }
            let res: String = sut.into();

            // Then
            assert!(res.contains('\u{FFFD}')); // Unicode replacement character
            assert_eq!(res, "Hello\u{FFFD}\u{FFFD}World!");
        }

        #[test]
        fn into_string_stops_at_first_nul() {
            // Given
            let bytes = b"first\0second";
            let mut sut = StringBuffer::with_capacity(bytes.len());
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr().cast::<c_char>(),
                    sut.as_mut_ptr(),
                    bytes.len(),
                );
            }

            // When
            let res: String = sut.into();

            // Then
            assert_eq!(res, "first");
        }

        #[test]
        fn into_string_without_nul_is_bounded_by_capacity() {
            // Given
            let bytes = b"complete buffer";
            let mut sut = StringBuffer::with_capacity(bytes.len());
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr().cast::<c_char>(),
                    sut.as_mut_ptr(),
                    bytes.len(),
                );
            }

            // When
            let res: String = sut.into();

            // Then
            assert_eq!(res, "complete buffer");
        }

        #[test]
        #[should_panic(expected = "string buffer capacity exceeds `c_int::MAX`")]
        fn with_capacity_exceeding_c_int_max_panics() {
            // Given
            let capacity = usize::try_from(c_int::MAX).unwrap() + 1;

            // When
            let _sut = StringBuffer::with_capacity(capacity);

            // Then
            // Constructor panics before attempting an allocation.
        }

        #[rstest]
        #[case("", None)]
        #[case(" ", None)]
        #[case("error message", Some(CoolPropError::Native("error message".into())))]
        fn into_coolprop_error(#[case] value: &str, #[case] expected: Option<CoolPropError>) {
            // Given
            let c_string = CString::new(value).unwrap();
            let c_bytes = c_string.as_bytes_with_nul();
            let mut sut = StringBuffer::with_capacity(c_bytes.len());

            // When
            unsafe {
                std::ptr::copy_nonoverlapping(
                    c_bytes.as_ptr().cast::<c_char>(),
                    sut.as_mut_ptr(),
                    c_bytes.len(),
                );
            }
            let res: Option<CoolPropError> = sut.into();

            // Then
            assert_eq!(res, expected);
        }
    }
}
