.section .text

/* _setup_interrupts
 *
 * We override
 * `PROVIDE(_setup_interrupts = default_setup_interrupts)`
 * of riscv-rt.
 *
 * Original `default_setup_interrupts` impl at riscv_rt::asm.S:175.
 */
.global _setup_interrupts
_setup_interrupts:
    // Set vector table
    la t0, ivt
    // Set vectored mode bit
    addi t0, t0, 1
    csrw mtvec, t0
    ret
