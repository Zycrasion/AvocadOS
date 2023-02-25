#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use avo_os::{QemuExitCode, exit_qemu, serial_println, serial::{serial_foreground, serial_reset_colour}, serial_print};

#[panic_handler]
fn panic(_info : &PanicInfo) -> !
{
    serial_foreground(0, 255, 0);
    serial_println!("[ok]");
    serial_reset_colour();
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
    should_fail();

    loop {};
}

fn should_fail()
{
    let title = "should_panic::should_fail";
    serial_print!("{}", title);
    let spaces_to_print = 100 - title.len() - 4;
    for _ in 0..spaces_to_print
    {
        serial_print!(".");
    }
    assert_eq!(1,0)
}

