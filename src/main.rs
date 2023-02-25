#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(avo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use avo_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
use avo_os::vga_buffer::{WRITER, ColorCode, Color};
    let code = WRITER.lock().color_code;
    WRITER.lock().color_code = ColorCode::new(Color::LightRed, Color::Black);
    println!("PANIC: {}", _info);
    WRITER.lock().color_code = code;
    avo_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avo_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("AvocadOS revision {}", 0);
    println!("version: {}.{}.{}", 0, 0, 0);
    
    avo_os::init();



    #[cfg(test)]
    test_main();

    avo_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}