extern crate libusb;

use std::time::Duration;

const VID_NZXT: u16 = 0x1e71;
const PID_KRAKEN_X62: u16 = 0x170e;
const PID_H500I: u16 = 0x1714;

struct UsbDevice<'a> {
  handle: libusb::DeviceHandle<'a>,
  language: libusb::Language,
  timeout: Duration,
}

fn main() {
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

      match device_desc.product_id() {
        PID_KRAKEN_X62 => println!(
          "Found NZXT Kraken X62: Bus {:03} Device {:03}",
          device.bus_number(),
          device.address()
        ),
        PID_H500I => println!(
          "Found NZXT H500i controller: Bus {:03} Device {:03}",
          device.bus_number(),
          device.address()
        ),
        _ => {
          let mut usb_device = {
            match device.open() {
              Ok(h) => match h.read_languages(timeout) {
                Ok(l) => {
                  if l.len() > 0 {
                    Some(UsbDevice {
                      handle: h,
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
          };
          println!(
            "Found unknown NZXT Device:{:04x} (iProduct: {:3} [{}])  at Bus {:03} Device {:03}",
            device_desc.vendor_id(),
            device_desc.product_string_index().unwrap_or(0),
            usb_device.as_mut().map_or(String::new(), |h| h
              .handle
              .read_product_string(h.language, &device_desc, h.timeout)
              .unwrap_or(String::new())),
            device.bus_number(),
            device.address()
          )
        }
      }
    }
  } else {
    println!("No NZXT devices found!");
  }
}
