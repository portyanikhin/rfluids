use core::ffi::{c_char, c_int, c_long};
use std::ffi::CString;
use std::sync::{LazyLock, Mutex};

pub(crate) static COOLPROP: LazyLock<Mutex<coolprop_sys::bindings::CoolProp>> =
    LazyLock::new(|| {
        Mutex::new(
            unsafe { coolprop_sys::bindings::CoolProp::new(coolprop_sys::COOLPROP_PATH) }
                .expect("Unable to load CoolProp dynamic library!"),
        )
    });

#[derive(Debug)]
pub(crate) struct ErrorBuffer {
    pub code: *mut c_long,
    pub message: MessageBuffer,
}

impl ErrorBuffer {
    pub(crate) fn blank() -> Self {
        Self {
            code: &mut 0,
            message: MessageBuffer::blank(),
        }
    }
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

impl MessageBuffer {
    fn with_capacity(capacity: c_int) -> Self {
        Self {
            capacity,
            buffer: CString::new(" ".repeat(capacity as usize))
                .unwrap()
                .into_raw(),
        }
    }

    pub(crate) fn blank() -> Self {
        Self::with_capacity(0)
    }
}

impl Default for MessageBuffer {
    fn default() -> Self {
        Self::with_capacity(500)
    }
}

impl From<MessageBuffer> for String {
    fn from(value: MessageBuffer) -> Self {
        unsafe { CString::from_raw(value.buffer).into_string().unwrap() }
    }
}

macro_rules! const_ptr_c_char {
    ($value:expr) => {
        format!("{}{}", $value, "\0").as_ptr() as *const c_char
    };
}

pub(crate) use const_ptr_c_char;
