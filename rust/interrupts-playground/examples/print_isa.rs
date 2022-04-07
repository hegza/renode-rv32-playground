#![no_std]
#![no_main]

#[macro_use]
extern crate common;

use bitvec::{order::Lsb0, view::BitView};
use heapless::{String, Vec};
use riscv_rt::entry;

fn decode_isa(reg_val: usize) -> Vec<char, 26> {
    let bits = reg_val.view_bits::<Lsb0>();
    let isa_ext = &bits[0..26];

    // Convert to iterator
    let isa_ext = isa_ext.iter().enumerate();

    let mut v = Vec::<_, 26>::new();

    for (n, b) in isa_ext {
        if *b {
            unsafe {
                v.push_unchecked((n as u8 + 0x41) as char);
            }
        }
    }
    v
}

#[entry]
fn main() -> ! {
    common::init_uart();

    use riscv::register as rv;

    let isa_arr = decode_isa(rv::misa::read().unwrap().bits());
    let mut isa_str = String::<32>::new();
    for c in isa_arr {
        isa_str.push(c).unwrap();
    }
    sprintln!("ISA: {}", &isa_str);

    loop {}
}
