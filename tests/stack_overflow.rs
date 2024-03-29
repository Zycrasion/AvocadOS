#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;

use avo_os::{serial::{serial_foreground, serial_reset_colour}, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let title = "stack_overflow::stack_overflow";
    serial_print!("{}", title);
    let len = 100 - 4 - title.len();
    for _ in 0..len {
        serial_print!(".");
    }

    avo_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Exectution persisted after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avo_os::test_panic_handler(info);
}

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(avo_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

use x86_64::structures::idt::InterruptStackFrame;
use avo_os::{exit_qemu, QemuExitCode};

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_foreground(0, 255, 0);
    serial_println!("[ok]");
    serial_reset_colour();
    exit_qemu(QemuExitCode::Success);
    loop {}
}
