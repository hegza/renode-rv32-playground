#![no_std]
#![no_main]

#[macro_use]
extern crate common;
use core::arch::asm;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;
use riscv_rt::entry;

#[export_name = "DefaultHandler"]
fn custom_interrupt_handler() {
    sprint!("INT:");
    sprintln!("mcause {}", riscv::register::mcause::read().bits());
}

#[entry]
fn main() -> ! {
    common::init_uart();

    sprintln!("Hello UART!");

    unsafe {
        use riscv::register::mie;
        mie::set_mtimer();

        let time = riscv::register::time::read();
        sprintln!("time: {}", &time);
        let next_time = time + 1_000;
        //asm!("csrw mtimecmp, {}", in(reg) next_time);

        let time = riscv::register::time::read();
        sprintln!("time: {}", &time);

        riscv::register::mip::set_mtimer();
    }
    sprintln!("Interrupt enabled?");

    let mut delay = riscv::delay::McycleDelay::new(50_000_000);
    for i in 0..3 {
        delay.delay_ms(1_000);
        sprintln!("{}", i + 1);
    }

    sprintln!(
        "mtvec location: 0x{:x}",
        riscv::register::mtvec::read().bits()
    );

    unsafe { riscv::asm::wfi() }

    sprintln!("Bottom code");

    loop {}
}
