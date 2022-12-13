#![allow(dead_code)]

use rusb::{Device, GlobalContext};

use crate::{Error, Result};

// OpenMoko Inc
const VENDOR: u16 = 0x1d50;
// Ubertooth One
const PRODUCT: u16 = 0x6002;

#[derive(Debug)]
pub struct Ubertooth {
    device: Device<GlobalContext>,
}

impl Ubertooth {
    /// Initialize the [`Ubertooth`].
    pub fn init() -> Result<Ubertooth> {
        if let Some(device) = rusb::devices()?.iter().try_find(|dev| {
            let desc = dev.device_descriptor()?;
            Ok::<bool, Error>(desc.vendor_id() == VENDOR && desc.product_id() == PRODUCT)
        })? {
            Ok(Ubertooth { device })
        } else {
            Err(Error::USB(rusb::Error::NoDevice))
        }
    }

    /// Return device version as `(major, minor, patch)`
    pub fn version(&self) -> Result<(u8, u8, u8)> {
        let v = self.device.device_descriptor()?.device_version();
        Ok((v.0, v.1, v.2))
    }
}
