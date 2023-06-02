// disable std lib
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

use core::panic::PanicInfo;

