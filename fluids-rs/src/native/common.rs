use core::ffi::{c_char, c_int, c_long};
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::sync::{LazyLock, Mutex};

pub(crate) static COOLPROP: LazyLock<Mutex<coolprop_sys::bindings::CoolProp>> =
    LazyLock::new(|| {
        Mutex::new(
            unsafe { coolprop_sys::bindings::CoolProp::new(coolprop_sys::COOLPROP_PATH) }
                .expect("Unable to load CoolProp dynamic library!"),
        )
    });

macro_rules! const_ptr_c_char {
    ($value:expr) => {
        format!("{}{}", $value, "\0").as_ptr() as *const c_char
    };
}

pub(crate) use const_ptr_c_char;

/// CoolProp internal error.
#[derive(Debug, Clone)]
pub struct CoolPropError(pub(crate) String);

impl std::error::Error for CoolPropError {}

impl Display for CoolPropError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) struct ErrorBuffer {
    pub code: *mut c_long,
    pub message: MessageBuffer,
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self {
            code: &mut 0,
            message: MessageBuffer::default(),
        }
    }
}

impl From<ErrorBuffer> for String {
    fn from(value: ErrorBuffer) -> Self {
        value.message.into()
    }
}

#[derive(Debug)]
pub(crate) struct MessageBuffer {
    pub capacity: c_int,
    pub buffer: *mut c_char,
}

impl Default for MessageBuffer {
    fn default() -> Self {
        let capacity = 500;
        Self {
            capacity: capacity as c_int,
            buffer: CString::new(" ".repeat(capacity)).unwrap().into_raw(),
        }
    }
}

impl From<MessageBuffer> for String {
    fn from(value: MessageBuffer) -> Self {
        unsafe { CString::from_raw(value.buffer).into_string().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const MESSAGE: &str = "Something went wrong...";

    #[test]
    fn coolprop_error_fmt_always_returns_error_message() {
        let sut = CoolPropError(MESSAGE.to_string());
        assert_eq!(format!("{}", sut), MESSAGE);
    }

    #[test]
    fn coolprop_error_to_string_always_returns_error_message() {
        let sut = CoolPropError(MESSAGE.to_string());
        assert_eq!(sut.to_string(), MESSAGE);
    }
}
