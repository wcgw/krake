extern crate libusb;

use std::process::exit;

//use libusb::Device;
//use libusb::Direction;
//use libusb::TransferType;
//use libusb::UsageType;

mod device;

//use crate::device::kraken;
//use crate::device::smart_device;
use crate::device::UsbDevice;
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
  //  let context = libusb::Context::new().unwrap();
  //  let first_device = find_first_device(smart_device::PRODUCT_ID, &context);
  //  match first_device {
  //    Some(device) => match device.active_config_descriptor() {
  //      Ok(config_desc) => {
  //        if config_desc.num_interfaces() != 1 {
  //          println!("Dunno what interface to choose here! :(");
  //          exit(1);
  //        }
  //        match config_desc.interfaces().last() {
  //          Some(inter) => {
  //            let desc = inter.descriptors().next().unwrap();
  //            for endpoint in desc.endpoint_descriptors() {
  //              if endpoint.direction() == Direction::In
  //                && endpoint.usage_type() == UsageType::Data
  //                && endpoint.transfer_type() == TransferType::Interrupt
  //              {
  //                match get_usb_device(&device) {
  //                  Some(usb_device) => {
  //                    let mut handle = usb_device.handle;
  //                    let claimed = handle.kernel_driver_active(inter.number()).unwrap();
  //                    if claimed {
  //                      println!("Detaching kernel driver!");
  //                      handle.detach_kernel_driver(inter.number()).unwrap();
  //                    }
  //                    match handle.claim_interface(inter.number()) {
  //                      Ok(()) => match handle.write_interrupt(endpoint.number(), &[0], usb_device.timeout) {
  //                        Ok(written) => {
  //                          println!("LEDs off! Wrote {} bytes", written);
  //                        },
  //                        Err(err) => {
  //                          println!("Failed! {}", err);
  //                          exit(1);
  //                        },
  //                      },
  //                      Err(err) => {
  //                        println!("Couldn't claim device: {}", err);
  //                        exit(1);
  //                      },
  //                    }
  //                    if claimed {
  //                      let result = handle.attach_kernel_driver(inter.number());
  //                      if result.is_err() {
  //                        println!("Error re attaching kernel driver: {}", result.err().unwrap())
  //                      }
  //                    }
  //                  },
  //                  None => {
  //                    println!("Couldn't open device!");
  //                    exit(1);
  //                  },
  //                }
  //              }
  //            }
  //          },
  //          None => {
  //            println!("No interface!");
  //            exit(1);
  //          },
  //        }
  //      },
  //      Err(err) => {
  //        println!("No active config: {}", err);
  //        exit(1);
  //      },
  //    },
  //    None => {
  //      println!("No device found!");
  //      exit(1);
  //    },
  //  }
}

fn leds_on() -> () {
  println!("LEDs on!")
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
          device.print_info();
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
