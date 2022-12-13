#![allow(dead_code)]

use rusb::{Device, DeviceList, GlobalContext};

use crate::Result;

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
        let device = DeviceList::new()?.iter().try_find(|dev| {
            let desc = dev.device_descriptor()?;
            if desc.vendor_id() == VENDOR && desc.product_id() == PRODUCT {
                Ok(true)
            } else {
                Err(rusb::Error::NoDevice)
            }
        })?;

        Ok(Ubertooth {
            device: device.expect("unexpected missing device"),
        })
    }

    /// Return device version as `(major, minor, patch)`
    pub fn version(&self) -> Result<(u8, u8, u8)> {
        let v = self.device.device_descriptor()?.device_version();
        Ok((v.0, v.1, v.2))
    }
}
