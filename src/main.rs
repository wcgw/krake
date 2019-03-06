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

      // Probably want to identify a device here first...
      // Not sure how this is best done tho? If only one, easy... if only one of one type?
      // A config file where one gets to id a device based of it's address and type?
      // e.g. `gpu` is the cooler on my graphic card? `case` is the Smart Device of my H500i?
      "leds" => match std::env::args().nth(2) {
        // Then, per action, ask the device if it known about e.g. the color scheme for LEDs?
        // Or default to all if not specified (works for `off, but what about e.g. `candle`?)
        // Only apply to the ones that resolved? Only error in case none resolved?
        // So that: `$ krake leds candle` would only affect the case, no-ops on Kraken
        // while: `$ krake leds breathing` would apply to both, so would `leds off`
        Some(state) => match state.as_str() {
          "on" => leds_on(),
          "off" => leds_off(),
          _ => {
            println!("Unknown state '{}' for LEDs!", state);
            exit(1);
          }
        },
        None => {
          println!("What state for your LEDs? (on|off)");
          exit(1);
        }
      },
      _ => {
        println!("Unsupported command: {}", command);
        exit(1);
      }
    },
    None => {
      println!("Please provide a command: (list|leds|version)");
      exit(1);
    }
  }
}

fn leds_off() -> () {
  println!("LEDs off!")
}

fn leds_on() -> () {
  println!("LEDs on!")
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
            device.address(),
          );
        }
        PID_SMART_DEVICE => println!(
          "Bus {:03} Device {:03}: NZXT Smart Device",
          device.bus_number(),
          device.address(),
        ),
        _ => println!(
          "Bus {:03} Device {:03}: Unknown NZXT Device: {:04x} (product: {})",
          device.bus_number(),
          device.address(),
          device_desc.product_id(),
          usb_device.as_mut().map_or("no access?!".to_owned(), |h| h
            .handle
            .read_product_string(h.language, &device_desc, h.timeout)
            .unwrap_or("unidentified".to_owned())),
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
