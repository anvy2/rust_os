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
    rust::init();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );
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
