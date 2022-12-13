#![feature(try_find)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/btbb.rs"));

mod error;
mod ubertooth;

pub use error::{Error, Result};

pub use crate::ubertooth::Ubertooth;
