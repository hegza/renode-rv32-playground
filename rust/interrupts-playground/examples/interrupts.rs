#![no_std]
#![no_main]

#[macro_use]
extern crate common;

use core::arch::global_asm;
use riscv::register::{mie, mip, mstatus};
use riscv_rt::{entry, TrapFrame};

global_asm!(include_str!("../asm/setup_interrupts.S"));
global_asm!(include_str!("../asm/ivt.S"));

const CORE_FREQ_MIPS: u32 = 100;
const CORE_FREQ: u32 = CORE_FREQ_MIPS * 1_000_000;

pub type CLINT = common::core_level_interruptor::CoreLevelInterruptor<0xF001_0000, 1>;

/// Machine software interrupt
#[no_mangle]
pub unsafe fn msoft() {
    riscv::interrupt::free(|| {
        // The UART type for printing comes from external context. I think the fact that this
        // works is UB :) The UART doesn't contain any variables so it works.
        sprintln!("msoft called");

        CLINT::complete(0);
    });
}

/// Machine timer interrupt
#[no_mangle]
pub unsafe fn mtimer() {
    riscv::interrupt::free(|| {
        // The UART type for printing comes from external context. I think the fact that this
        // works is UB :) The UART doesn't contain any variables so it works.
        sprintln!("mtimer called");

        let seconds = 1;
        CLINT::set_time_cmp(0, CLINT::time() + CORE_FREQ as u64 * seconds);
    });
}

#[no_mangle]
pub fn exception(_frame: TrapFrame) {
    loop {
        continue;
    }
}

#[entry]
fn main() -> ! {
    common::init_uart();

    unsafe {
        // Allow core to respond to msoft and mtimer
        mie::set_msoft();
        mie::set_mtimer();

        // Allow core to respond to interrupts in general
        mstatus::set_mie();
    }

    // Raise software interrupt for HART 0
    CLINT::raise(0);

    sprintln!("Setting timer for 1 second");
    let seconds = 1;
    CLINT::set_time_cmp(0, CLINT::time() + CORE_FREQ as u64 * seconds);

    sprintln!("wfi");
    unsafe { riscv::asm::wfi() };

    loop {}
}
