use crate::device::{Device, UsbDevice};

pub const PRODUCT_ID: u16 = 0x1714;

pub struct SmartDevice {
  usb_device: UsbDevice,
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

impl SmartDevice {
  pub fn new(usb_device: UsbDevice) -> Self {
    SmartDevice { usb_device }
  }

  pub fn leds(&mut self, color: Color) -> Result<(), String> {
    let mut data = led_message();

    // 5..35 controls the first strip
    for i in (5..35).step_by(3) {
      data[i] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }
    // 35..62 controls second strip -1
    for i in (35..62).step_by(3) {
      data[i] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }
    // 65..68 controls the last led
    for i in (65..68).step_by(3) {
      data[i] = color.g; // G
      data[i + 1] = color.r; // R
      data[i + 2] = color.b; // B
    }

    self.usb_device.write(&data)
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
