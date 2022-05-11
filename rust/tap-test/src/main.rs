#![no_std]
#![no_main]

#[macro_use]
extern crate common;

use riscv_rt::entry;

#[entry]
unsafe fn main() -> ! {
    common::init_uart();
    sprintln!("Hello, world!");

    loop {}
}
