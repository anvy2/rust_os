#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use rust::allocator::HEAP_SIZE;

extern crate alloc;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use rust::allocator;
    use rust::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    rust::init();
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap allocation failed");
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(534);
    let heap_value_2 = Box::new(234);
    assert_eq!(*heap_value_1, 534);
    assert_eq!(*heap_value_2, 234);
}

#[test_case]
fn large_vector() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<i64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..2 * HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
}
