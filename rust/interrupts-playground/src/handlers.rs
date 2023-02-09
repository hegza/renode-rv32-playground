use riscv::register::{mepc, mip};

use crate::{CLINT, CORE_FREQ};

/// Machine software interrupt
/// 3: RISC-V specification for smclic (CLIC ) says msip/msoft is interrupt id 3
#[no_mangle]
pub unsafe fn msoft() {
    riscv::interrupt::free(|| {
        let this_pc = mepc::read();

        // The UART type for printing comes from external context. I think the fact that this
        // works is UB :) The UART doesn't contain any variables so it works.
        sprintln!("msi, mepc: 0x{:x}", this_pc);

        CLINT::complete(0);
    });
}

/// Machine timer interrupt
/// 7: RISC-V specification for smclic (CLIC ) says mtip is interrupt id 7
#[no_mangle]
pub unsafe fn mtimer() {
    riscv::interrupt::free(|| {
        let this_pc = mepc::read();

        // The UART type for printing comes from external context. I think the fact that this
        // works is UB :) The UART doesn't contain any variables so it works.
        sprintln!("mti, mepc: 0x{:x}", this_pc);

        let seconds = 1;
        CLINT::set_time_cmp(0, CLINT::time() + CORE_FREQ as u64 * seconds);
    });
}

#[export_name = "exception"]
pub fn exception(trap_frame: &riscv_rt::TrapFrame) -> ! {
    sprintln!("ExceptionHandler. Trap frame: {:?}", trap_frame);
    loop {
        continue;
    }
}
