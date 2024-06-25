#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(try_find)]

include!(concat!(env!("OUT_DIR"), "/btbb.rs"));

mod error;
mod ubertooth;
mod usb;

pub use error::{Error, Result};
pub use usb::{Info, Led};

pub use crate::ubertooth::{count, Ubertooth};
