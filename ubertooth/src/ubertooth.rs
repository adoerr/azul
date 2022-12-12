#![allow(dead_code)]

use rusb::{Device, UsbContext};

// OpenMoko Inc
const VENDOR: u16 = 0x1d50;
// Ubertooth One
const PRODUCT: u16 = 0x6002;

#[derive(Debug)]
pub struct Ubertooth<T: UsbContext> {
    device: Device<T>
}

impl<T> Ubertooth<T>
where
T: UsbContext,
{
    /// Initialize the [`Ubertooth`].
    pub fn init() -> Ubertooth<T> {
        todo!()
    }
}