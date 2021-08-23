#![no_std]
#![cfg_attr(test, no_main)]

use iotxxx_nrf52832_kit as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
