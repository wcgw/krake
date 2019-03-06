use std::time::Duration;

pub mod kraken;
pub mod smart_device;

pub const NZXT_PID: u16 = 0x1e71;

pub struct UsbDevice<'a> {
  pub handle: libusb::DeviceHandle<'a>,
  pub language: libusb::Language,
  pub timeout: Duration,
}

pub fn nzxt_devices() -> () {}
