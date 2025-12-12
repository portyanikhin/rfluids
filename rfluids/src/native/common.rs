use core::ffi::{c_char, c_int, c_long};
use std::{ffi::CString, sync::MutexGuard};

use super::CoolPropError;
use crate::io::GlobalParam;

#[derive(Debug)]
pub(crate) struct ErrorBuffer {
    pub code: *mut c_long,
    pub message: MessageBuffer,
}

impl ErrorBuffer {
    pub fn blank() -> Self {
        Self { code: &mut 0, message: MessageBuffer::blank() }
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self { code: &mut 0, message: MessageBuffer::default() }
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
    pub fn with_capacity(capacity: c_int) -> Self {
        Self { capacity, buffer: CString::new(" ".repeat(capacity as usize)).unwrap().into_raw() }
    }

    pub fn blank() -> Self {
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

pub(crate) fn get_error(
    lock: &MutexGuard<coolprop_sys::bindings::CoolProp>,
) -> Option<CoolPropError> {
    let message = MessageBuffer::default();
    let _unused = unsafe {
        lock.get_global_param_string(
            const_ptr_c_char!(GlobalParam::PendingError.as_ref()),
            message.buffer,
            message.capacity,
        )
    };
    let res: String = message.into();
    if res.trim().is_empty() { None } else { Some(CoolPropError(res)) }
}

macro_rules! const_ptr_c_char {
    ($value:expr) => {
        format!("{}{}", $value, "\0").as_ptr().cast::<core::ffi::c_char>()
    };
}

pub(crate) use const_ptr_c_char;
