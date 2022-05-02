use core::arch::global_asm;

global_asm!(include_str!("../asm/setup_interrupts.S"));
global_asm!(include_str!("../asm/ivt.S"));
