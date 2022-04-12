use volatile_register::RW;

/// Driver for IRQControllers.CoreLevelInterruptor device in Renode.
///
/// # Type arguments
///
/// * `P` memory-mapped address
/// * `H` number of 'targets' or HARTs.
///
/// Implementation is based on Renode version 1.12.0.36865.
pub struct CoreLevelInterruptor<const P: usize, const H: usize>(pub(crate) ());

impl<const P: usize, const H: usize> CoreLevelInterruptor<P, H> {
    const PTR: *const RegisterBlock = P as *const _;
}

// TODO: extend & test to 64-bit.
/// Register block for renode's IRQControllers.CoreLevelInterruptor. 32-bit only.
/// Size = 0x1_0000. Register positions as implemented in Renode CoreLevelInterruptor:
///
/// * MSipHart0 = 0x0 * 4 * hartid,
/// * MTimeCmpHart0Lo = 0x4000 * 8 * hartid,
/// * MTimeCmpHart0Hi = 0x4004 * 8 * hartid,
/// * MTimeLo = 0xBFF8
/// * MTimeHi = 0xBFFC
#[repr(C)]
pub struct RegisterBlock {
    /// Software or inter-processor (IPI), per-HART, machine-mode, max 2048 instances
    /// 0x0     -- 0x2000
    pub msip: [RW<u32>; 0x800],

    /// 0x2000  -- 0x4000: padding
    #[doc(hidden)]
    _padding1: [u32; 0x800],

    /// Timer compare lo and hi, per-HART, machine-mode, max 2048 instances
    /// 0x4000  -- 0x8000
    pub mtime_cmp: [RW<u64>; 0x800],

    /// 0x8000  -- 0xBFF8: padding
    #[doc(hidden)]
    _padding2: [u32; 0xFFE],

    /// Fixed-frequency counter, machine-mode
    /// 0xBFF8  -- 0xC000
    pub mtime: RW<u64>,

    /// 0xC000  -- 0x10000: padding
    #[doc(hidden)]
    _padding3: [u32; 0x1000],
}

impl<const P: usize, const H: usize> CoreLevelInterruptor<P, H> {
    /// Raises a software interrupt for `target` HART
    ///
    /// # Arguments
    ///
    /// * `target`: target HART ID
    #[inline]
    pub fn raise(target: usize) {
        unsafe { (*Self::PTR).msip[target].modify(|v| v | 0b1) };
    }

    /// Completes / lowers a software interrupt for `target` HART
    ///
    /// # Arguments
    ///
    /// * `target`: target HART ID
    #[inline]
    pub fn complete(target: usize) {
        unsafe { (*Self::PTR).msip[target].modify(|v| v & !0b1) };
    }

    /// Returns the timer counter value
    pub fn time() -> u64 {
        unsafe { (*Self::PTR).mtime.read() }
    }

    /// Sets a value for when timer interrupt is triggered
    pub fn set_time_cmp(target: usize, value: u64) {
        unsafe { (*Self::PTR).mtime_cmp[target].write(value) };
    }
}
