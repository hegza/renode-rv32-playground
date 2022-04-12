/* norvc :=
 *  Enables or disables the generation of compressed instructions. Instructions are
 *  opportunistically compressed by the RISC-V assembler when possible, but sometimes this behavior
 *  is not desirable.
 */
.option norvc

//.section .vectors
.global ivt
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

/*
    Machine software interrupt trampoline

    Stores caller saved registers on stack and restores them after the interrupt.
 */
.align 4
.global _msi_trampoline
_msi_trampoline:
    // Save return address on stack
    addi sp, sp, -2*4
    sw ra, 0*4(sp)
    sw t0, 1*4(sp)

    // Surround interrupt handler with store/load frame
    jal t0, _store_frame
    jal ra, msoft
    jal t0, _load_frame

    // Pop return address from stack
    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    addi sp, sp, 2*4
    mret

/*
    Machine timer interrupt trampoline

    Stores caller saved registers on stack and restores them after the interrupt.
 */
.align 4
.global _mti_trampoline
_mti_trampoline:
    // Save return address on stack
    addi sp, sp, -2*4
    sw ra, 0*4(sp)
    sw t0, 1*4(sp)

    // Surround interrupt handler with store/load frame
    jal t0, _store_frame
    jal ra, mtimer
    jal t0, _load_frame

    // Pop return address from stack
    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    addi sp, sp, 2*4
    mret

// Stores the stack frame, reserving alternate link register (x5/t0) for the
// return address as per RISC-V standard calling convention.
.align 4
.global _store_frame
_store_frame:
    addi sp, sp, -14*4

    // DO NOT STORE ra  // return address
    // DO NOT STORE t0  // alternate link register
    sw t1, 0*4(sp)      // temporary #1
    sw t2, 1*4(sp)      // temporary #2
    sw t3, 2*4(sp)      // temporary #3
    sw t4, 3*4(sp)      // temporary #4
    sw t5, 4*4(sp)      // temporary #5
    sw t6, 5*4(sp)      // temporary #6
    sw a0, 6*4(sp)      // function arguments / return values
    sw a1, 7*4(sp)      // function arguments / return values
    sw a2, 8*4(sp)      // function arguments
    sw a3, 9*4(sp)      // function arguments
    sw a4, 10*4(sp)     // function arguments
    sw a5, 11*4(sp)     // function arguments
    sw a6, 12*4(sp)     // function arguments
    sw a7, 13*4(sp)     // function arguments

    jr t0

// Restores the stack frame, reserving alternate link register (x5/t0) for the
// return address as per RISC-V standard calling convention.
.align 4
.global _load_frame
_load_frame:
    // DO NOT LOAD ra   // return address
    // DO NOT LOAD t0   // alternate link register
    lw t1, 0*4(sp)
    lw t2, 1*4(sp)
    lw t3, 2*4(sp)
    lw t4, 3*4(sp)
    lw t5, 4*4(sp)
    lw t6, 5*4(sp)
    lw a0, 6*4(sp)
    lw a1, 7*4(sp)
    lw a2, 8*4(sp)
    lw a3, 9*4(sp)
    lw a4, 10*4(sp)
    lw a5, 11*4(sp)
    lw a6, 12*4(sp)
    lw a7, 13*4(sp)

    addi sp, sp, 14*4
    jr t0


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
