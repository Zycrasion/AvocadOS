use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, vga_buffer::{WRITER, ColorCode, Color}};

use lazy_static::lazy_static;

lazy_static!
{
    static ref IDT : InterruptDescriptorTable = 
    {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        return idt
    };
}

pub fn init_idt()
{
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame : InterruptStackFrame
)
{
    let colour = WRITER.lock().color_code;
    WRITER.lock().color_code = ColorCode::new(Color::LightRed, Color::Black);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    WRITER.lock().color_code = colour;
}

#[test_case]
fn test_breakpoint_exception()
{
    x86_64::instructions::interrupts::int3();
}