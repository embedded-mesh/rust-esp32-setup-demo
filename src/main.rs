#![allow(clippy::single_component_path_imports)]

use anyhow::{bail, Result};

use log::*;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;


#[allow(dead_code)]
const SSID: &str = "abc";
#[allow(dead_code)]
const PASS: &str = "ctie";


fn main() -> Result<()> {
    test_print();

    #[allow(unused)]
    let peripherals = Peripherals::take().unwrap();
    // let pins = peripherals.pins;
    // let gpio1 = pins.gpio0.into_input()?;
    // println!("More complex print {:?}", gpio1.);

    Ok(())
}

#[allow(clippy::vec_init_then_push)]
fn test_print() {
    // Start simple
    println!("Hello from Rust!");

    // Check collections
    let mut children = vec![];

    children.push("foo");
    children.push("bar");
    println!("More complex print {:?}", children);
}
