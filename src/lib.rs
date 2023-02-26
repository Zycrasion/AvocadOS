#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

use crate::{serial::{serial_foreground, serial_reset_colour}, vga_buffer::{WRITER, ColorCode, Color}};

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;

extern crate alloc;

pub fn init()
{
    let colour = WRITER.lock().color_code;
    WRITER.lock().color_code = ColorCode::new(Color::Green, Color::Black);
    println!("Starting AvocadOS Initialisation");

    println!("Initialising Global Descriptor Table!");
    gdt::init();

    println!("Initialising Interrupt Descriptor Table!");
    interrupts::init_idt();

    println!("Initialising Programmable Interrupt Controller (Intel 8259)");
    unsafe {interrupts::PICS.lock().initialize();}

    println!("Enabling interrupts");
    x86_64::instructions::interrupts::enable();

    println!("Initialisation Finished!");
 
    WRITER.lock().color_code = colour;
}

#[alloc_error_handler]
fn alloc_error_handler(layout : alloc::alloc::Layout) -> !
{
    panic!("allocation error: {:?}", layout);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        let name = core::any::type_name::<T>();
        let ok_msg = "[ok]";
        let spaces_to_print = (100 - name.len()) - ok_msg.len();
        serial_print!("{}", name);
        for _ in 0..spaces_to_print
        {
            serial_print!(".");
        }
        self();
        serial_foreground(0, 255, 0);
        serial_println!("{}",ok_msg);
        serial_reset_colour();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!();
    serial_println!();

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_foreground(200, 0, 0);
    serial_println!("[failed]");
    serial_foreground(255, 0, 0);
    serial_println!("PANIC: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info : &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn hlt_loop() -> !
{
    loop
    {
        x86_64::instructions::hlt();
    }
}