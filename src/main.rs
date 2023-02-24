#![no_std]
#![no_main]
mod vga_buffer;
use vga_buffer::{Color, ColorCode, Writer, Buffer};
use core::panic::{PanicInfo};
use core::fmt::Write;

#[panic_handler]
fn panic(_info : &PanicInfo) -> !
{
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> !
{
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("a");

    writeln!(writer, "test {}", 2).unwrap();

    loop {}
}