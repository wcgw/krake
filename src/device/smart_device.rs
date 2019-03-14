use crate::device::UsbDevice;
use libusb::{Direction, TransferType, UsageType};

pub const PRODUCT_ID: u16 = 0x1714;

pub struct SmartDevice<'a> {
  usb_device: UsbDevice<'a>,
}

pub enum LedState {
  On,
  Off,
}

impl<'a> SmartDevice<'a> {
  pub fn new(usb_device: UsbDevice<'a>) -> Self {
    SmartDevice { usb_device }
  }

  pub fn leds(&mut self, state: LedState) -> Result<(), String> {
    match state {
      LedState::Off => {
        let mut data: [u8; 128] = [0; 128];
        data[0] = 0x2;
        data[1] = 0x4b;
        data[4] = 0x2;
        data[64] = 0x3;
        self.message(&mut data)
      },
      LedState::On => {
        let mut data: [u8; 128] = [0; 128];

        for i in 5..35 { // 5..35 controls the first strip
          data[i] = 0xff;
        }
        for i in 35..62 { // 35..62 controls second strip -1
          data[i] = 0xff;
        }
        for i in 65..68 { // 65..68 controls the last led
          data[i] = 0xff;
        }

        data[0] = 0x2;
        data[1] = 0x4b;
        data[4] = 0x2;
        data[64] = 0x3;
        self.message(&mut data)
      },
    }
  }

  fn message(&mut self, data: &mut [u8; 128]) -> Result<(), String> {
    match self.usb_device.device.active_config_descriptor() {
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
                let handle = &mut self.usb_device.handle;
                let claimed = handle.kernel_driver_active(inter.number()).unwrap();
                if claimed {
                  println!("Detaching kernel driver!");
                  handle.detach_kernel_driver(inter.number()).unwrap();
                }
                match handle.claim_interface(inter.number()) {
                  Ok(()) => {
                    println!("Writing to endpoint {}", endpoint.number());
                    match handle.write_interrupt(endpoint.number(), data, self.usb_device.timeout) {
                      Ok(written) => {
                        println!("Wrote {} bytes", written);
                      },
                      Err(err) => {
                        return Err(format!("Failed! {}", err));
                      },
                    }
                  },
                  Err(err) => {
                    return Err(format!("Couldn't claim device: {}", err));
                  },
                }
                if claimed {
                  let result = handle.attach_kernel_driver(inter.number());
                  if result.is_err() {
                    println!("Error re attaching kernel driver: {}", result.err().unwrap())
                  }
                }
              }
            }
          },
          None => {
            return Err(format!("No interface!"));
          },
        }
      },
      Err(err) => {
        return Err(format!("No active config: {}", err));
      },
    }
    Ok(())
  }
}
