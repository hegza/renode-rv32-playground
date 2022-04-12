#![no_std]

pub mod core_level_interruptor;
mod println;

use uart_16550::MmioSerialPort;

const SERIAL_PORT_BASE_ADDRESS: usize = 0x6000_1800;

pub struct Shared {
    pub serial_port: Option<MmioSerialPort>,
}

pub static mut SHARED: Shared = Shared { serial_port: None };

/// Make sprintln work.
pub fn init_uart() {
    let mut serial_port = unsafe { MmioSerialPort::new(SERIAL_PORT_BASE_ADDRESS) };
    serial_port.init();
    unsafe {
        SHARED.serial_port = Some(serial_port);
    }
}

// Write error to UART on panic
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("{}", info);

    loop {}
}
