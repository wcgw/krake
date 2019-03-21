pub mod kraken;
pub mod smart_device;

pub const NZXT_PID: u16 = 0x1e71;

pub struct DeviceManager {
  context: hidapi::HidApi,
}

impl DeviceManager {
  pub fn new() -> Result<Self, String> {
    match hidapi::HidApi::new() {
      Ok(context) => Ok(DeviceManager { context }),
      Err(err) => Err(err.to_string()),
    }
  }

  pub fn all(&self) -> Vec<Result<UsbDevice, String>> {
    let mut devices = vec![];
    for device in self.context.devices().iter() {
      if device.vendor_id == NZXT_PID {
        let usb_device: Result<UsbDevice, String> = device.open_device(&self.context).try_into(device.product_id);

        match usb_device {
          Ok(dev) => devices.push(Ok(dev)),
          Err(msg) => {
            let manufacturer = device.manufacturer_string.clone().unwrap_or("unknown".to_owned());
            let product = device.product_string.clone().unwrap_or("unknown".to_owned());
            devices.push(Err(format!(
              "Couldn't open device at {} ({} by {}): {}",
              device.path.to_str().unwrap_or("<unparsable path>"),
              manufacturer,
              product,
              msg
            )))
          },
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

pub struct UsbDevice {
  device: hidapi::HidDevice,
  product_id: u16,
}

impl<'a> Device for UsbDevice {
  fn print_info(&self) -> () {
    match self.product_id {
      kraken::X62::PRODUCT_ID => println!(
        "NZXT Kraken X62 [s/n: {}]",
        self
          .device
          .get_serial_number_string()
          .unwrap()
          .unwrap_or("unknown".to_owned()),
      ),
      smart_device::PRODUCT_ID => println!(
        "NZXT Smart Device [s/n: {}]",
        self
          .device
          .get_serial_number_string()
          .unwrap()
          .unwrap_or("unknown".to_owned()),
      ),
      _ => println!(
        "Unknown {} Device: {:04x} (product: {})",
        self
          .device
          .get_manufacturer_string()
          .unwrap()
          .unwrap_or("unknown".to_owned()),
        self.product_id,
        self
          .device
          .get_product_string()
          .unwrap()
          .unwrap_or("unknown".to_owned()),
      ),
    }
  }

  fn device_id(&self) -> u16 {
    self.product_id
  }

  fn write(&mut self, data: &[u8]) -> Result<(), String> {
    let mut vec = vec![];
    vec.extend_from_slice(data);
    match self.device.write(&vec) {
      Ok(written) => println!("Wrote {} bytes to endpoint", written),
      Err(err) => return Err(err.to_string()),
    };
    Ok(())
  }
}

trait TryInto<T> {
  fn try_into(self: Self, id: u16) -> Result<T, String>;
}

impl TryInto<UsbDevice> for hidapi::HidResult<hidapi::HidDevice> {
  fn try_into(self, id: u16) -> Result<UsbDevice, String> {
    match self {
      Ok(device) => Ok(UsbDevice { device, product_id: id }),
      Err(err) => Err(err.to_string()),
    }
  }
}
