#![no_main]
#![no_std]

use iotxxx_nrf52832_kit as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main");

    defmt::panic!()
}
