#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(avo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::{boxed::Box, vec::Vec, vec, rc::Rc};
use avo_os::{memory::{self, BootInfoFrameAllocator}, println, allocator};
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::{
    structures::paging::{Page},
    VirtAddr,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use avo_os::vga_buffer::{Color, ColorCode, WRITER};
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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("AvocadOS revision {}", 0);
    println!("version: {}.{}.{}", 0, 0, 0);

    avo_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let _page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    allocator::init_heap(&mut mapper,&mut frame_allocator).expect("heap initialisation failed!");

    println!("type ~ to enter kernel-debug mode");

    #[cfg(test)]
    test_main();

    avo_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
