use std::time::Duration;

pub mod kraken;
pub mod smart_device;

pub const NZXT_PID: u16 = 0x1e71;

pub trait Device {
  fn print_info(&self) -> ();
}

pub struct UsbDevice<'a> {
  device: libusb::Device<'a>,
  handle: libusb::DeviceHandle<'a>,
  language: libusb::Language,
  timeout: Duration,
}

impl <'a> UsbDevice<'a> {
  pub fn all(context: & libusb::Context) -> Vec<UsbDevice> {
    let mut devices = vec![];
    for device in context.devices().unwrap().iter() {
      let device_desc = device.device_descriptor().unwrap();
      if device_desc.vendor_id() == NZXT_PID {
        let usb_device = as_usb_device(device);
        if usb_device.is_some() {
          devices.push(usb_device.unwrap());
        }
      }
    }
    devices
  }
}

impl <'a> Device for UsbDevice<'a> {
  fn print_info(&self) -> () {
    let device_desc = self.device.device_descriptor().unwrap();

    match device_desc.product_id() {
      kraken::x62::PRODUCT_ID => {
        println!(
          "Bus {:03} Device {:03}: NZXT Kraken X62",
          self.device.bus_number(),
          self.device.address(),
        );
      },
      smart_device::PRODUCT_ID => println!(
        "Bus {:03} Device {:03}: NZXT Smart Device",
        self.device.bus_number(),
        self.device.address(),
      ),
      _ => println!(
        "Bus {:03} Device {:03}: Unknown NZXT Device: {:04x} (product: {})",
        self.device.bus_number(),
        self.device.address(),
        device_desc.product_id(),
        self.handle
            .read_product_string(self.language, &device_desc, self.timeout)
            .unwrap_or("unidentified".to_owned())),
    }
  }
}

fn as_usb_device(device: libusb::Device) -> Option<UsbDevice> {
  let timeout = Duration::from_millis(200);
  match device.open() {
    Ok(handle) => match handle.read_languages(timeout) {
      Ok(l) => {
        if l.len() > 0 {
          Some(UsbDevice {
            device,
            handle,
            language: l[0],
            timeout,
          })
        } else {
          None
        }
      },
      Err(_) => None,
    },
    Err(_) => None,
  }
}
