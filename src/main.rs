extern crate libusb;

use std::process::exit;

//use libusb::Device;
//use libusb::Direction;
//use libusb::TransferType;
//use libusb::UsageType;

mod device;

//use crate::device::kraken;
//use crate::device::smart_device;
use crate::device::smart_device::LedState;
use crate::device::smart_device::SmartDevice;
use crate::device::{Device, DeviceManager};

fn main() {
  match std::env::args().nth(1) {
    Some(command) => match command.as_str() {
      "version" => println!("Krake v0.0.1 - Controls for NZXT bells & whistles"),
      "list" => list_nzxt_devices(),

      // Probably want to identify a device here first...
      // Not sure how this is best done tho? If only one, easy... if only one of one type?
      // A config file where one gets to id a device based of it's address and type?
      // e.g. `gpu` is the cooler on my graphic card? `case` is the Smart Device of my H500i?
      "leds" => match std::env::args().nth(2) {
        // Then, per action, ask the device if it known about e.g. the color scheme for LEDs?
        // Or default to all if not specified (works for `off, but what about e.g. `candle`?)
        // Only apply to the ones that resolved? Only error in case none resolved?
        // So that: `$ krake leds candle` would only affect the case, no-ops on Kraken
        // while: `$ krake leds breathing` would apply to both, so would `leds off`
        Some(state) => match state.as_str() {
          "on" => leds_on(),
          "off" => leds_off(),
          _ => {
            println!("Unknown state '{}' for LEDs!", state);
            exit(1);
          },
        },
        None => {
          println!("What state for your LEDs? (on|off)");
          exit(1);
        },
      },
      _ => {
        println!("Unsupported command: {}", command);
        exit(1);
      },
    },
    None => {
      println!("Please provide a command: (list|leds|version)");
      exit(1);
    },
  }
}

fn leds_off() -> () {
  match DeviceManager::new() {
    Ok(device_manager) => {
      let devices = device_manager.all();

      if devices.len() > 0 {
        for device in devices {
          match device {
            Ok(device) => {
              if device.device_id() == device::smart_device::PRODUCT_ID {
                let mut smart_device = SmartDevice::new(device);
                match smart_device.leds(LedState::Off) {
                  Err(err) => {
                    println!("Couldn't turn LEDs off: {}", err);
                    exit(1)
                  },
                  Ok(()) => println!("LEDs off!"),
                }
              }
            },
            Err(msg) => println!("Error: {}", msg),
          }
        }
      } else {
        println!("No NZXT devices found!");
      }
    },
    Err(msg) => {
      println!("Couldn't create DeviceManager: {}", msg);
      exit(1)
    },
  }
}

fn leds_on() -> () {
  match DeviceManager::new() {
    Ok(device_manager) => {
      let devices = device_manager.all();

      if devices.len() > 0 {
        for device in devices {
          match device {
            Ok(device) => {
              if device.device_id() == device::smart_device::PRODUCT_ID {
                let mut smart_device = SmartDevice::new(device);
                match smart_device.leds(LedState::On) {
                  Err(err) => {
                    println!("Couldn't turn LEDs on: {}", err);
                    exit(1)
                  },
                  Ok(()) => println!("LEDs on!"),
                }
              }
            },
            Err(msg) => println!("Error: {}", msg),
          }
        }
      } else {
        println!("No NZXT devices found!");
      }
    },
    Err(msg) => {
      println!("Couldn't create DeviceManager: {}", msg);
      exit(1)
    },
  }
}

//fn find_first_device(product_id: u16, context: &Context) -> Option<Device> {
//  for device in context.devices().unwrap().iter() {
//    let device_desc = device.device_descriptor().unwrap();
//    if device_desc.vendor_id() == device::NZXT_PID {
//      if device_desc.product_id() == product_id {
//        return Some(device);
//      }
//    }
//  }
//  None
//}

fn list_nzxt_devices() -> () {
  match DeviceManager::new() {
    Ok(device_manager) => {
      let devices = device_manager.all();

      if devices.len() > 0 {
        for device in devices {
          match device {
            Ok(device) => device.print_info(),
            Err(msg) => println!("Error: {}", msg),
          }
        }
      } else {
        println!("No NZXT devices found!");
      }
    },
    Err(msg) => {
      println!("Couldn't create DeviceManager: {}", msg);
      exit(1)
    },
  }
}
