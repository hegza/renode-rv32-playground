/* norvc :=
 *  Enables or disables the generation of compressed instructions. Instructions are
 *  opportunistically compressed by the RISC-V assembler when possible, but sometimes this behavior
 *  is not desirable.
 */
.option norvc

.global ivt
// Align the vector table to a multiple of 4 bytes. This ensures the last two bits of the address
// are 0, which is mandated by the structure of the mtvec as specified in RISC-V.
.align 4
ivt:
    .org  ivt + 0*4;
    j   _exception_trampoline;  /* 0 */
    //.org  ivt + 1*4;
    //j   ssi;      /* 1  */
    .org  ivt + 3*4;
    j   _msi_trampoline;        /* 3 */
    //.org  ivt + 5*4;
    //j   sti;  /* 5  */
    .org  ivt + 7*4;
    j   _mti_trampoline;        /* 7 */
    //.org  ivt + 9*4;
    //j   sei;  /* 9  */
    //.org  ivt + 11*4;
    //j   mei;  /* 11 */
    /*
    .org  ivt + 16*4;
    j   platform_irq0;
    j   platform_irq1;
    j   platform_irq2;
    j   platform_irq3;
    j   platform_irq4;
    j   platform_irq5;
    j   platform_irq6;
    j   platform_irq7;
    j   platform_irq8;
    j   platform_irq9;
    j   platform_irq10;
    j   platform_irq11;
    j   platform_irq12;
    j   platform_irq13;
    j   platform_irq14;
    j   platform_irq15;
    */

// Stores the stack frame of context
.macro STORE_FRAME
    //  Allocate space in stack
    addi sp, sp, -17*4

    sw x1, 0*4(sp)      // ra, return address
    sw x4, 1*4(sp)      // tp, thread pointer
    sw x5, 2*4(sp)      // t0, alternate link register

    // t1-t2, temporary registers
    sw x6, 3*4(sp)
    sw x7, 4*4(sp)

    // a0-a1, function argument, return value registers
    sw x10, 5*4(sp)
    sw x11, 6*4(sp)

    // a2-a7, function argument registers
    sw x12, 7*4(sp)
    sw x13, 8*4(sp)
    sw x14, 9*4(sp)
    sw x15, 10*4(sp)
    sw x16, 11*4(sp)
    sw x17, 12*4(sp)

    // t3-t6 temporary registers
    sw x28, 13*4(sp)
    sw x29, 14*4(sp)
    sw x30, 15*4(sp)
    sw x31, 16*4(sp)
.endm

// Restores the stack frame of context
.macro RESTORE_FRAME
    lw x1, 0*4(sp)      // ra, return address
    lw x4, 1*4(sp)      // tp, thread pointer
    lw x5, 2*4(sp)      // t0, alternate link register

    // t1-t2, temporary registers
    lw x6, 3*4(sp)
    lw x7, 4*4(sp)

    // a0-a1, function argument, return value registers
    lw x10, 5*4(sp)
    lw x11, 6*4(sp)

    // a2-a7, function argument registers
    lw x12, 7*4(sp)
    lw x13, 8*4(sp)
    lw x14, 9*4(sp)
    lw x15, 10*4(sp)
    lw x16, 11*4(sp)
    lw x17, 12*4(sp)

    // t3-t6 temporary registers
    lw x28, 13*4(sp)
    lw x29, 14*4(sp)
    lw x30, 15*4(sp)
    lw x31, 16*4(sp)

    // De-allocate the stack space
    addi sp, sp, 17*4
.endm

/*
    Machine software interrupt trampoline

    Stores caller saved registers on stack and restores them after the interrupt.
 */
.align 4
.global _msi_trampoline
_msi_trampoline:
    // Surround interrupt handler with store/load frame
    STORE_FRAME
    jal ra, msoft
    RESTORE_FRAME

    mret

/*
    Machine timer interrupt trampoline

    Stores caller saved registers on stack and restores them after the interrupt.
 */
.align 4
.global _mti_trampoline
_mti_trampoline:
    // Surround interrupt handler with store/load frame
    STORE_FRAME
    jal ra, mtimer
    RESTORE_FRAME

    mret

/*
    Exception handler

    Stores caller saved registers and calls the exception handler.
 */
.align 4
.global _exception_trampoline
_exception_trampoline:
    addi sp, sp, -16*4

    sw ra, 0*4(sp)      // return address
    sw t0, 1*4(sp)      // alternate link
    sw t1, 2*4(sp)      // temporary #1
    sw t2, 3*4(sp)      // temporary #2
    sw t3, 4*4(sp)      // temporary #3
    sw t4, 5*4(sp)      // temporary #4
    sw t5, 6*4(sp)      // temporary #5
    sw t6, 7*4(sp)      // temporary #6
    sw a0, 8*4(sp)      // function arguments / return values
    sw a1, 9*4(sp)      // function arguments / return values
    sw a2, 10*4(sp)     // function arguments
    sw a3, 11*4(sp)     // function arguments
    sw a4, 12*4(sp)     // function arguments
    sw a5, 13*4(sp)     // function arguments
    sw a6, 14*4(sp)     // function arguments
    sw a7, 15*4(sp)     // function arguments

    // Save frame as parameter. Binary layout matches TrapFrame -type
    add a0, sp, zero

    j exception
