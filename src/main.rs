#![no_std]
#![no_main]
mod vga_buffer;
use vga_buffer::{WRITER, Color, ColorCode};
use core::panic::{PanicInfo};


#[panic_handler]
fn panic(_info : &PanicInfo) -> !
{
    let code = WRITER.lock().color_code;
    WRITER.lock().color_code = ColorCode::new(Color::LightRed, Color::Black);
    println!("PANIC: {}", _info);
    WRITER.lock().color_code = code;
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> !
{
    println!("AvocadOS revision {}", 0);
    println!("version: {}.{}.{}", 0 , 0 , 0);


    loop {}
}