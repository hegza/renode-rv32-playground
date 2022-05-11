#![no_std]
#![no_main]

#[macro_use]
extern crate common;

use riscv::register::mhartid;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    common::init_uart();

    use riscv::register as rv;

    assert_eq!(mhartid(), 0, "mhartid must return 0 on boot core");
    assert_eq!(mhartid(), 1, "intended failure");

    loop {}
}
