use crate::device::{Device, FanStatus, FirmwareStatus, NoiseStatus, PowerStatus, Status, UsbDevice};

pub const PRODUCT_ID: u16 = 0x1714;
pub const SPEED_CHANNELS: u8 = 3;

pub struct SmartDevice {
  usb_device: UsbDevice,
}

pub enum LedDevice {
    Hue,
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

    self.usb_device.write(&data).map(|_| ())
  }

  pub fn status(&mut self) -> Vec<Status> {
      // Report:
      // Byte  | Meaning
      // ------|-----------
      //   00  | ?
      //   01  | Noise Level
      //   02  | ?
      //   03  | RPM in multiples of 256
      //   04  | RPM units
      //   05  | ?
      //   06  | ?
      //   07  | Voltage
      //   08  | Voltage pt 2
      //   09  | ?
      //   10  | Amp draw
      //   11  | Firmware Version Major
      //   12  | Firmware Version Minor
      //   13  | Firmware Version Minor pt 2
      //   14  | Firmware Version Patchlevel
      //   15  | 2 least significant bits used for power type, rest for sensor number
      //   16  | LED accessory type
      //   17  | LED count
    let mut statuses = vec![];

    let buffers =
        (0..SPEED_CHANNELS).map(|_| {
            let mut buf: [u8; 21] = [0; 21];
            self.usb_device.read(&mut buf);
            buf
        }).collect::<Vec<[u8; 21]>>();

    buffers.first().map(|buf| {
       statuses.push(Status::Firmware(FirmwareStatus {
           major: buf[11] as u16,
           minor: (buf[12] as u16) << 8 | (buf[13] as u16),
           patch: buf[14] as u16,
       }));

       statuses.push(Status::Noise(NoiseStatus { db: buf[1] }));

       let led_count = buf[17];

       if led_count > 0 {
           let led_type = match buf[16] {
               0x00 | 0x01 => LedDevice::Hue,
               other => panic!("Unknown device {}", other),
           };
       };
    });

    for buf in buffers {
      // println!(
      //   "{:?}",
      //   buf.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>()
      // );
      let num = buf[15] >> 3 + 1;
      let speed_type = buf[15] & 0x03;
      let rpm = (buf[3] as u16) << 8 | (buf[4] as u16);
      let mv = (buf[7] as u16) * 100 + (buf[8] as u16);
      let ma = (buf[10] as u16) * 10;

      let status = match speed_type {
        0x00 => Status::UnpoweredFan,
        0x01 => Status::Fan(FanStatus {
          power: PowerStatus::DC,
          ma: ma,
          mv: mv,
          rpm: rpm,
        }),
        0x02 => Status::Fan(FanStatus {
          power: PowerStatus::PWM,
          ma: ma,
          mv: mv,
          rpm: rpm,
        }),
        other => unreachable!("Unknown speed type {:02x}", other),
      };

      println!("Fan {:02x}, {:?}", num, status);
      statuses.push(status);
    }
    statuses
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
