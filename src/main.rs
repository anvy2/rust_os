#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use rust::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world {}", "45");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust::test_panic_handler(info)
}
