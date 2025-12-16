use core::ffi::{c_char, c_int, c_long};
use std::{cell::Cell, ffi::CString, marker::PhantomData, sync::MutexGuard};

use super::CoolPropError;
use crate::io::GlobalParam;

/// Marker to make structs `!Send + !Sync` for thread safety.
type PhantomUnsync = PhantomData<Cell<()>>;

#[derive(Debug)]
pub(crate) struct ErrorBuffer {
    code: c_long,
    pub message: MessageBuffer,
    marker: PhantomUnsync,
}

impl ErrorBuffer {
    pub fn blank() -> Self {
        Self { code: 0, message: MessageBuffer::blank(), marker: PhantomData }
    }

    #[must_use]
    pub fn code_as_mut_ptr(&mut self) -> *mut c_long {
        &raw mut self.code
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn code(&self) -> c_long {
        self.code
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self { code: 0, message: MessageBuffer::default(), marker: PhantomData }
    }
}

impl From<ErrorBuffer> for Option<CoolPropError> {
    fn from(value: ErrorBuffer) -> Self {
        value.message.into()
    }
}

#[derive(Debug)]
pub(crate) struct MessageBuffer {
    capacity: usize,
    buffer: *mut c_char,
    marker: PhantomUnsync,
}

impl MessageBuffer {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self { capacity, buffer: std::ptr::null_mut(), marker: PhantomData };
        }
        let vec = vec![0u8; capacity];
        let buffer = unsafe { CString::from_vec_unchecked(vec) }.into_raw();
        Self { capacity, buffer, marker: PhantomData }
    }

    #[must_use]
    pub fn blank() -> Self {
        Self::with_capacity(0)
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.buffer
    }

    #[must_use]
    pub fn capacity(&self) -> c_int {
        self.capacity as c_int
    }
}

impl Default for MessageBuffer {
    fn default() -> Self {
        Self::with_capacity(500)
    }
}

impl Drop for MessageBuffer {
    fn drop(&mut self) {
        if !self.buffer.is_null() {
            unsafe {
                drop(CString::from_raw(self.buffer));
            }
        }
    }
}

impl From<MessageBuffer> for String {
    fn from(value: MessageBuffer) -> Self {
        if value.buffer.is_null() {
            return Self::new();
        }
        let buffer = value.buffer;
        std::mem::forget(value);
        let c_string = unsafe { CString::from_raw(buffer) };
        c_string.into_string().unwrap_or_else(|e| e.into_cstring().to_string_lossy().into_owned())
    }
}

impl From<MessageBuffer> for Option<CoolPropError> {
    fn from(value: MessageBuffer) -> Self {
        let message: String = value.into();
        if message.trim().is_empty() { None } else { Some(CoolPropError(message)) }
    }
}

pub(crate) fn get_error(
    lock: &MutexGuard<coolprop_sys::bindings::CoolProp>,
) -> Option<CoolPropError> {
    let mut message = MessageBuffer::default();
    let param = CString::new(GlobalParam::PendingError.as_ref()).unwrap();
    let _unused = unsafe {
        lock.get_global_param_string(param.as_ptr(), message.as_mut_ptr(), message.capacity())
    };
    message.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod message_buffer {
        use rstest::*;

        use super::*;

        #[rstest]
        #[case(0)]
        #[case(42)]
        fn with_capacity(#[case] capacity: usize) {
            // When
            let sut = MessageBuffer::with_capacity(capacity);

            // Then
            assert_eq!(sut.capacity(), capacity as c_int);
        }

        #[test]
        fn blank() {
            // When
            let sut = MessageBuffer::blank();

            // Then
            assert_eq!(sut.capacity(), 0);
        }

        #[test]
        fn default() {
            // When
            let sut = MessageBuffer::default();

            // Then
            assert_eq!(sut.capacity(), 500);
        }

        #[rstest]
        #[case("")]
        #[case("something")]
        #[case(" something else ")]
        fn into_string(#[case] value: &str) {
            // Given
            let c_string = CString::new(value).unwrap();
            let c_bytes = c_string.as_bytes_with_nul();
            let mut sut = MessageBuffer::with_capacity(c_bytes.len());

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
            let sut = MessageBuffer::with_capacity(42);

            // When
            let res: String = sut.into();

            // Then
            assert!(res.is_empty());
        }

        #[test]
        fn into_string_blank() {
            // Given
            let sut = MessageBuffer::blank();

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
            let mut sut = MessageBuffer::with_capacity(invalid_utf8.len());

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

        #[rstest]
        #[case("", None)]
        #[case(" ", None)]
        #[case("Error message", Some(CoolPropError("Error message".into())))]
        fn into_coolprop_error(#[case] value: &str, #[case] expected: Option<CoolPropError>) {
            // Given
            let c_string = CString::new(value).unwrap();
            let c_bytes = c_string.as_bytes_with_nul();
            let mut sut = MessageBuffer::with_capacity(c_bytes.len());

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
