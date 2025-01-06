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

macro_rules! const_ptr_c_char {
    ($value:expr) => {
        format!("{}{}", $value, "\0").as_ptr() as *const c_char
    };
}

pub(crate) use const_ptr_c_char;
