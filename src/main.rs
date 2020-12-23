#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(abacus_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use abacus_os::println;
use bootloader::{BootInfo, entry_point};
use alloc::boxed::Box;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    abacus_os::init();

    // Testing dummy allocator
    let x = Box::new(41);

    #[cfg(test)]
        test_main();

    println!("No crashing!");
    abacus_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    abacus_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    abacus_os::test_panic_handler(info)
}