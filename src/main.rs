#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(avo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use avo_os::{println, memory};

use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr, structures::paging::Translate};

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

entry_point!(kernel_main);

fn kernel_main(boot_info : &'static BootInfo) -> ! {
    println!("AvocadOS revision {}", 0);
    println!("version: {}.{}.{}", 0, 0, 0);

    avo_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = 
    [
        0xb8000,
        0x201008,
        0x100_00201a10,
        boot_info.physical_memory_offset
    ];

    for &address in &addresses
    {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} => {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    avo_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}