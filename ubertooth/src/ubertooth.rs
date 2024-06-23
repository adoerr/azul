#![allow(dead_code)]

use std::time::Duration;

use rusb::{
    request_type, DeviceDescriptor, DeviceHandle, DeviceList, Direction, GlobalContext, Recipient,
    RequestType,
};

use crate::{
    usb::{Commands, Led},
    Error, Result,
};

// OpenMoko Inc
const VENDOR: u16 = 0x1d50;
// Ubertooth One
const PRODUCT: u16 = 0x6002;

#[derive(Debug)]
pub struct Ubertooth {
    descriptor: DeviceDescriptor,
    handle: DeviceHandle<GlobalContext>,
}

impl Ubertooth {
    /// Initialize and open the [`Ubertooth`].
    ///
    /// Using the first Ubertooth Once device found.
    pub fn start() -> Result<Ubertooth> {
        if let Some(device) = DeviceList::new()?.iter().try_find(|dev| {
            let desc = dev.device_descriptor()?;
            Ok::<bool, Error>(desc.vendor_id() == VENDOR && desc.product_id() == PRODUCT)
        })? {
            Ok(Ubertooth {
                descriptor: device.device_descriptor()?,
                handle: device.open()?,
            })
        } else {
            Err(Error::USB(rusb::Error::NoDevice))
        }
    }

    /// Return [`Ubertooth`] version as `(major, minor, patch)`
    pub fn version(&self) -> Result<(u8, u8, u8)> {
        let v = self.descriptor.device_version();
        Ok((v.0, v.1, v.2))
    }

    /// Read user LED state.
    pub fn user_led(&self) -> Result<u8> {
        let mut state = [0u8; 1];

        self.handle.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Endpoint),
            Commands::GET_USRLED as u8,
            0,
            0,
            &mut state,
            Duration::from_millis(10),
        )?;

        Ok(u8::from_ne_bytes(state))
    }

    /// Set uer LED state.
    pub fn set_user_led(&self, state: Led) -> Result<()> {
        self.handle.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Endpoint),
            Commands::SET_USRLED as u8,
            state as u16,
            0,
            &[],
            Duration::from_millis(10),
        )?;

        Ok(())
    }

    /// Enable Christmas lights effect.
    pub fn xmas_lights(&self) -> Result<()> {
        self.handle.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Endpoint),
            Commands::XMAS as u8,
            0,
            0,
            &[],
            Duration::from_millis(10),
        )?;

        Ok(())
    }
}

/// Count the number of [`Ubertooth`] devices connected.
pub fn count() -> Result<usize> {
    DeviceList::new()?.iter().try_fold(0, |acc, dev| {
        let desc = dev.device_descriptor()?;
        if desc.vendor_id() == VENDOR && desc.product_id() == PRODUCT {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
}
