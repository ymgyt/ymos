#![no_std] // Without os support.
#![no_main] // #![no_main]: No entrypoint we have.

use core::panic::PanicInfo;

// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Do not mangle to tell linker the name of this function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

fn main() {}
