#![no_std]
#![no_main]

#[macro_use]
extern crate common;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    common::init_uart();

    sprintln!("Hello UART!");

    loop {}
}
