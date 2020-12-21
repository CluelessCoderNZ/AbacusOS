#![no_std] // Cannot use std library due to OS hooks
#![no_main] // Since the std library is not included. We must self-define the start point
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static MESSAGE: &[u8] = b"Hello World!";

// Entry point for kernel
#[no_mangle] // Forces function symbol to be called _start in linker
pub extern "C" fn _start() -> ! {

    // Write Hello World to vga buffer. This method is written as unsafe as a quick
    // test of our bootloader but can/will be written in a safe context in future.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}