extern crate libusb;

use std::process::exit;
use std::time::Duration;

use libusb::Device;

const VID_NZXT: u16 = 0x1e71;

const PID_KRAKEN_X62: u16 = 0x170e;
const PID_SMART_DEVICE: u16 = 0x1714;

struct UsbDevice<'a> {
  handle: libusb::DeviceHandle<'a>,
  language: libusb::Language,
  timeout: Duration,
}

fn main() {
  match std::env::args().nth(1) {
    Some(command) => match command.as_str() {
      "version" => println!("Krake v0.0.1 - Controls for NZXT bells & whistles"),
      "list" => list_nzxt_devices(),
      _ => {
        println!("Unsupported command: {}", command);
        exit(1);
      }
    },
    None => {
      println!("Please provide a command: list, version");
      exit(1);
    }
  }
}

fn list_nzxt_devices() -> () {
  let context = libusb::Context::new().unwrap();
  let timeout = Duration::from_secs(1);
  let mut devices = vec![];

  for device in context.devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();
    if device_desc.vendor_id() == VID_NZXT {
      devices.push(device)
    }
  }

  if devices.len() > 0 {
    for device in devices {
      let device_desc = device.device_descriptor().unwrap();
      let mut usb_device = get_device(&device, timeout);

      match device_desc.product_id() {
        PID_KRAKEN_X62 => {
          println!(
            "Bus {:03} Device {:03}: NZXT Kraken X62",
            device.bus_number(),
            device.address()
          );
        }
        PID_SMART_DEVICE => println!(
          "Bus {:03} Device {:03}: NZXT Smart Device",
          device.bus_number(),
          device.address()
        ),
        _ => println!(
          "Bus {:03} Device {:03}: Unknown NZXT Device:{:04x} (product: {:3} [{}])",
          device.bus_number(),
          device.address(),
          device_desc.vendor_id(),
          device_desc.product_string_index().unwrap_or(0),
          usb_device.as_mut().map_or("unknown".to_owned(), |h| h
            .handle
            .read_product_string(h.language, &device_desc, h.timeout)
            .unwrap_or("unidentified".to_owned()))
        ),
      }
    }
  } else {
    println!("No NZXT devices found!");
  }
}

fn get_device<'a>(device: &'a Device, timeout: Duration) -> Option<UsbDevice<'a>> {
  match device.open() {
    Ok(handle) => match handle.read_languages(timeout) {
      Ok(l) => {
        if l.len() > 0 {
          Some(UsbDevice {
            handle,
            language: l[0],
            timeout,
          })
        } else {
          None
        }
      }
      Err(_) => None,
    },
    Err(_) => None,
  }
}
