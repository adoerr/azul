use anyhow::Result;
use rusb::DeviceList;
use usb_ids::{self, FromId};

const VENDOR: u16 = 0x1d50;
const PRODUCT: u16 = 0x6002;

fn main() -> Result<()> {
    for device in DeviceList::new()?.iter() {
        let desc = device.device_descriptor()?;
        if desc.vendor_id() == VENDOR && desc.product_id() == PRODUCT {
            println!("found Ubertooth `{:?}`", device);

            let vendor_name = match usb_ids::Vendor::from_id(desc.vendor_id()) {
                Some(vendor) => vendor.name(),
                None => "Unknown vendor"
            };

            print!("vendor `{vendor_name}` ");

            let product_name = match usb_ids::Device::from_vid_pid(desc.vendor_id(), desc.product_id()) {
                Some(product) => product.name(),
                None => "Unknown product"
            };

            println!("product `{product_name}`");
        }
    }

    Ok(())
}