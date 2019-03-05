extern crate libusb;

const VID_NZXT: u16 = 0x1e71;
const PID_KRAKEN_X62: u16 = 0x170e;
const PID_H500I: u16 = 0x1714;

fn main() {
  let context = libusb::Context::new().unwrap();

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
        _ => println!(
          "Found unknown NZXT Device:{:04x} at Bus {:03} Device {:03}",
          device_desc.vendor_id(),
          device.bus_number(),
          device.address()
        ),
      }
    }
  } else {
    println!("No NZXT devices found!");
  }
}
