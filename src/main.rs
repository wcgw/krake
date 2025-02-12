extern crate clap;
extern crate hidapi;

mod device;

use crate::device::smart_device::Color;
use crate::device::smart_device::SmartDevice;
use crate::device::{Device, DeviceManager};

use clap::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  let cmdline = Command::new("Krake")
    .version(VERSION)
    .author("The wcgw team - github.com/wcgw")
    .about("Controls for NZXT bells and whistles")
    .subcommand_required(true)
    .subcommand(Command::new("list").display_order(1).about("Lists all devices"))
    .subcommand(
      Command::new("leds")
        .display_order(2)
        .about("Controls the LEDs")
        .subcommand_required(true)
        .subcommand(Command::new("off").display_order(1))
        .subcommand(Command::new("white").display_order(2))
        .subcommand(Command::new("red").display_order(3))
        .subcommand(Command::new("green").display_order(4))
        .subcommand(Command::new("blue").display_order(5)),
    );

  match cmdline.get_matches().subcommand() {
    Some(("list", _)) => list_nzxt_devices(),
    Some(("leds", sub)) => match sub.subcommand() {
      Some(("off", _)) => leds(Color::off()),
      Some(("red", _)) => leds(Color::red()),
      Some(("green", _)) => leds(Color::green()),
      Some(("blue", _)) => leds(Color::blue()),
      Some(("white", _)) => leds(Color::white()),
      Some((color, _)) => unreachable!("Unknown color for leds '{}'", color),
      None => unreachable!("Should provide a subcommand"),
    },
    Some((cmd, _)) => unreachable!("Unknown subcommand '{}'", cmd),
    None => unreachable!("Should provide a subcommand"),
  }
}

fn leds(color: Color) {
  match DeviceManager::new() {
    Ok(device_manager) => {
      let devices = device_manager.all();

      if !devices.is_empty() {
        for device in devices {
          match device {
            Ok(device) => {
              if device.device_id() == device::smart_device::PRODUCT_ID {
                let mut smart_device = SmartDevice::new(device);
                if let Err(err) = smart_device.leds(color.clone()) {
                  println!("Couldn't change LEDs: {}", err);
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
    },
  }
}

fn list_nzxt_devices() {
  match DeviceManager::new() {
    Ok(device_manager) => {
      let devices = device_manager.all();

      if !devices.is_empty() {
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
    },
  }
}
