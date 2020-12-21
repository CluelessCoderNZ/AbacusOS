#![no_std] // Cannot use std library due to OS hooks
#![no_main] // Since the std library is not included. We must self-define the start point
use core::panic::PanicInfo;
mod vga_text;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Entry point for kernel
#[no_mangle] // Forces function symbol to be called _start in linker
pub extern "C" fn _start() -> ! {
    panic!("Hello World{}", "!");

    loop {}
}