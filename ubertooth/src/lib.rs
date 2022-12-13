#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/btbb.rs"));

mod ubertooth;
mod error;

pub use crate::ubertooth::Ubertooth;
pub use error::{Result, Error};