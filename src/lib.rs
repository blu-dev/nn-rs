#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate bitflags;

pub mod mem;
pub mod fs;
pub mod os;
pub mod timespan;
pub use timespan::TimeSpan;
pub mod vi;

#[macro_use]
extern crate nn_macro;

pub use nn_macro::*;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Result(u32);

impl Result {
    pub fn new(module: u32, description: u32) -> Self {
        let description = (description & 0b0001_1111_1111_1111) << 9;
        let module = module & 0b0001_1111_1111;
        Self(description | module)
    }

    pub fn get(self) -> (u32, u32) {
        let inner = self.0;
        let module = inner & 0b0001_1111_1111;
        let description = (module & (0b0001_1111_1111_1111 << 9)) >> 9;
        (module, description)
    }

    pub fn is_success(&self) -> bool {
        self.0 == 0
    }
}

#[macro_export]
macro_rules! get_rust_result {
    ($nx:ident, $ok:expr) => {
        if $nx.is_success() {
            Ok($ok)
        } else {
            Err($nx)
        }
    }
}

#[macro_export]
macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

pub fn from_c_str(c_str: *const libc::c_char) -> core::result::Result<String, core::str::Utf8Error> {
    unsafe {
        let name_slice = core::slice::from_raw_parts(c_str as *const _, libc::strlen(c_str));
        match core::str::from_utf8(name_slice) {
            Ok(v) => Ok(v.to_owned()),
            Err(e) => Err(e)
        }
    }
}