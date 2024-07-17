extern crate env_logger;
extern crate log;
extern crate rusb;

use log::info;
use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc901; // Litra Beam product ID
const ENDPOINT: u8 = 0x01;
const TIMEOUT_MS: Duration = Duration::from_millis(3000);
const LIGHT_ON: u8 = 0x01;
const LIGHT_OFF: u8 = 0x00;

fn main() {
    // look for the cli arg of on or off
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).unwrap();
    env_logger::init();
    let context = Context::new().unwrap();

    let mut device_handle: Option<DeviceHandle<Context>> = None;

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
            info!("Found Litra Beam");
            let device = device.open().expect("Could not open device");
            device_handle = Some(device);
            break;
        }
    }

    if let Some(mut handle) = device_handle {
        if let Ok(true) = handle.kernel_driver_active(0) {
            handle.detach_kernel_driver(0).unwrap();
        }

        power(&handle, command == "on");
    } else {
        info!("Litra Beam not found");
    }
}

fn power(handle: &DeviceHandle<Context>, on: bool) {
    handle.set_active_configuration(1).unwrap();
    handle.claim_interface(0).unwrap();

    let on_off = if on { LIGHT_ON } else { LIGHT_OFF };
    let data = [
        0x11, 0xff, 0x04, 0x1c, on_off, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    handle.write_bulk(ENDPOINT, &data, TIMEOUT_MS).unwrap();
    handle.release_interface(0).unwrap();
    handle.attach_kernel_driver(0).unwrap();
}
