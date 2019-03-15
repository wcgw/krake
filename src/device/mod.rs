use std::time::Duration;
use libusb::{Direction, UsageType, TransferType};

pub mod kraken;
pub mod smart_device;

pub const NZXT_PID: u16 = 0x1e71;

pub struct DeviceManager {
  context: libusb::Context,
}

impl DeviceManager {
  pub fn new() -> Result<Self, &'static str> {
    match libusb::Context::new() {
      Ok(context) => Ok(DeviceManager { context }),
      Err(err) => Err(err.strerror()),
    }
  }

  pub fn all(&self) -> Vec<Result<UsbDevice, String>> {
    let mut devices = vec![];
    for device in self.context.devices().unwrap().iter() {
      let device_desc = device.device_descriptor().unwrap();
      if device_desc.vendor_id() == NZXT_PID {
        let bus = device.bus_number();
        let addr = device.address();
        let usb_device: Result<UsbDevice, &str> = device.try_into();
        match usb_device {
          Ok(dev) => devices.push(Ok(dev)),
          Err(msg) => devices.push(Err(format!("Couldn't open device at {:03}:{:03}: {}", bus, addr, msg))),
        }
      }
    }
    devices
  }
}

pub trait Device {
  fn print_info(&self) -> ();
  fn device_id(&self) -> u16;
  fn write(&mut self, data: &[u8]) -> Result<(), String>;
}

pub struct UsbDevice<'a> {
  device: libusb::Device<'a>,
  handle: libusb::DeviceHandle<'a>,
  language: libusb::Language,
  timeout: Duration,
}

impl<'a> Device for UsbDevice<'a> {
  fn print_info(&self) -> () {
    let device_desc = self.device.device_descriptor().unwrap();

    match device_desc.product_id() {
      kraken::X62::PRODUCT_ID => println!(
        "Bus {:03} Device {:03}: NZXT Kraken X62 [s/n: {}]",
        self.device.bus_number(),
        self.device.address(),
        self
          .handle
          .read_serial_number_string(self.language, &device_desc, self.timeout)
          .unwrap_or("unknown".to_owned()),
      ),
      smart_device::PRODUCT_ID => println!(
        "Bus {:03} Device {:03}: NZXT Smart Device [s/n: {}]",
        self.device.bus_number(),
        self.device.address(),
        self
          .handle
          .read_serial_number_string(self.language, &device_desc, self.timeout)
          .unwrap_or("unknown".to_owned()),
      ),
      _ => println!(
        "Bus {:03} Device {:03}: Unknown NZXT Device: {:04x} (product: {})",
        self.device.bus_number(),
        self.device.address(),
        device_desc.product_id(),
        self
          .handle
          .read_product_string(self.language, &device_desc, self.timeout)
          .unwrap_or("unidentified".to_owned())
      ),
    }
  }

  fn device_id(&self) -> u16 {
    self.device.device_descriptor().unwrap().product_id()
  }

  fn write(&mut self, data: &[u8]) -> Result<(), String> {
    match self.device.active_config_descriptor() {
      Ok(config_desc) => {
        if config_desc.num_interfaces() != 1 {
          return Err("Dunno what interface to choose here! :(".to_owned());
        }
        match config_desc.interfaces().last() {
          Some(inter) => {
            let desc = inter.descriptors().next().unwrap();
            for endpoint in desc.endpoint_descriptors() {
              if endpoint.direction() == Direction::In
                && endpoint.usage_type() == UsageType::Data
                && endpoint.transfer_type() == TransferType::Interrupt
              {
                let handle = &mut self.handle;
                let claimed = handle.kernel_driver_active(inter.number()).unwrap();
                if claimed {
                  println!("Detaching kernel driver!");
                  handle.detach_kernel_driver(inter.number()).unwrap();
                }
                match handle.claim_interface(inter.number()) {
                  Ok(()) => match handle.write_interrupt(endpoint.number(), data, self.timeout) {
                    Ok(written) => {
                      println!(
                        "Wrote {} bytes to endpoint {} [0x{:x}]",
                        written,
                        endpoint.number(),
                        endpoint.address()
                      );
                    }
                    Err(err) => {
                      return Err(format!("Failed! {}", err));
                    }
                  },
                  Err(err) => {
                    return Err(format!("Couldn't claim device: {}", err));
                  }
                }
                if claimed {
                  let result = handle.attach_kernel_driver(inter.number());
                  if result.is_err() {
                    println!("Error re attaching kernel driver: {}", result.err().unwrap())
                  }
                }
              }
            }
          }
          None => {
            return Err(format!("No interface!"));
          }
        }
      }
      Err(err) => {
        return Err(format!("No active config: {}", err));
      }
    }
    Ok(())
  }
}

trait TryInto<T> {
  fn try_into(self: Self) -> Result<T, &'static str>;
}

impl<'a> TryInto<UsbDevice<'a>> for libusb::Device<'a> {
  fn try_into(self) -> Result<UsbDevice<'a>, &'static str> {
    let timeout = Duration::from_millis(200);
    match self.open() {
      Ok(handle) => match handle.read_languages(timeout) {
        Ok(l) => {
          if l.len() > 0 {
            Ok(UsbDevice {
              device: self,
              handle,
              language: l[0],
              timeout,
            })
          } else {
            Err("No language")
          }
        }
        Err(err) => Err(err.strerror()),
      },
      Err(err) => Err(err.strerror()),
    }
  }
}
