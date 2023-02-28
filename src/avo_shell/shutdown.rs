use core::arch::asm;

use alloc::string::String;
use x86_64::instructions::port::Port;

use crate::{println, exit_qemu, QemuExitCode};

use super::env::{MODE, AvocadOSMode};

pub fn shutdown_cmd(command_cache : &String)
{
    let md = MODE.lock().value;
    match md
    {
        AvocadOSMode::QEMU => 
        {
            exit_qemu(QemuExitCode::Success);
        },  
        _ => {
            println!("UNSUPPORTED HOST")
        }
    }
}