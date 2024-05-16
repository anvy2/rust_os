#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust::println;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust::{
        allocator,
        memory::{self, BootInfoFrameAllocator},
    };
    println!("Hello World!");
    rust::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    #[cfg(test)]
    test_main();
    println!("It did not crash");
    rust::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust::test_panic_handler(info)
}
