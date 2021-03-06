#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(abacus_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use abacus_os::{println, memory, allocator};
use bootloader::{BootInfo, entry_point};

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use x86_64::VirtAddr;
use abacus_os::memory::BootInfoFrameAllocator;
use abacus_os::task::{Task, keyboard};
use abacus_os::task::executor::Executor;

entry_point!(kernel_main);

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    abacus_os::init();

    // Initialise Heap
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");


    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

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