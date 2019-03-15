use crate::device::UsbDevice;
use libusb::{Direction, TransferType, UsageType};

pub const PRODUCT_ID: u16 = 0x1714;

pub struct SmartDevice<'a> {
  usb_device: UsbDevice<'a>,
}

#[derive(Clone, Debug)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn off() -> Color {
    Color { r: 0, g: 0, b: 0 }
  }

  pub fn white() -> Color {
    Color {
      r: 0xff,
      g: 0xff,
      b: 0xff,
    }
  }

  pub fn red() -> Color {
    Color {
      r: 0xff,
      g: 0x00,
      b: 0x00,
    }
  }

  pub fn green() -> Color {
    Color {
      r: 0x00,
      g: 0xff,
      b: 0x00,
    }
  }

  pub fn blue() -> Color {
    Color {
      r: 0x00,
      g: 0x00,
      b: 0xff,
    }
  }
}

impl<'a> SmartDevice<'a> {
  pub fn new(usb_device: UsbDevice<'a>) -> Self {
    SmartDevice { usb_device }
  }

  pub fn leds(&mut self, color: Color) -> Result<(), String> {
    let mut data = led_message();

    // 5..35 controls the first strip
    for i in (5..35).step_by(3) {
      data[i + 0] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }
    // 35..62 controls second strip -1
    for i in (35..62).step_by(3) {
      data[i + 0] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }
    // 65..68 controls the last led
    for i in (65..68).step_by(3) {
      data[i + 0] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }

    self.write(&data)
  }

  fn write(&mut self, data: &[u8; 128]) -> Result<(), String> {
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

fn led_message() -> [u8; 128] {
  let mut data: [u8; 128] = [0; 128];
  data[0] = 0x02; // document me
  data[1] = 0x4b; // LEDs
  data[2] = 0x00; // document me
  data[3] = 0x00; // document me
  data[4] = 0x02; // document me
                  // [GRB] colors for 19 leds
  data[63] = 0x00; // WTF?
  data[64] = 0x03; // WTF?
  data[65] = 0x00; // WTF?
                   // [GRB] last led here
  data
}
