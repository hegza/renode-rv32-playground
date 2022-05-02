#![no_std]
#![no_main]

#[macro_use]
extern crate common;

mod asm;
mod handlers;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use riscv::register::{mie, mstatus};
use riscv_rt::entry;

const CORE_FREQ_MIPS: u32 = 100;
const CORE_FREQ: u32 = CORE_FREQ_MIPS * 1_000_000;

pub type CLINT = common::core_level_interruptor::CoreLevelInterruptor<0xF001_0000, 1>;
//const PLIC: *mut usize = 0x50000000 as *mut usize;
/*const PRIO_BITS: usize = 3;
pub type PLIC = plic::Plic<0x50000000, PRIO_BITS>;*/

#[entry]
unsafe fn main() -> ! {
    common::init_uart();

    //sprintln!("MPP: {:?}", mstatus::read().mpp());
    sprintln!("Hello UART!");

    // Timer lo should be at 0xF001BFF8
    sprintln!("timer: {}", CLINT::time());

    let mut delay = riscv::delay::McycleDelay::new(CORE_FREQ);
    delay.delay_ms(500);

    sprintln!("Enable interrupts!");

    // Allow core to respond to msoft and mtimer
    mie::set_msoft();
    mie::set_mtimer();

    // Allow core to respond to interrupts in general
    mstatus::set_mie();

    // CLINT reg[0] |= 0b1 generates the interrupt from CLINT@0 -> CPU@X
    CLINT::raise(0);

    // TODO: Set up a timer too
    sprintln!("Setting timer for 1 second");
    let seconds = 1;
    CLINT::set_time_cmp(0, CLINT::time() + CORE_FREQ as u64 * seconds);

    sprintln!("pre-wfi");
    {
        riscv::asm::wfi();
    }
    sprintln!("post-wfi");

    /*
    unsafe {
        // PLIC
        let hart_id = mhartid::read();
        sprintln!("{}", hart_id);
        let threshold = PLIC::get_threshold(hart_id);
        let irq = PLIC::claim(hart_id).unwrap();
        let prio = PLIC::get_priority(irq);
    }

    sprintln!("Bottom code");
    */
    loop {}
}
