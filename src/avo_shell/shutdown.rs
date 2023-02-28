

use alloc::string::String;


use crate::{println, exit_qemu, QemuExitCode};

use super::env::{MODE, AvocadOSMode};

pub fn shutdown_cmd(_command_cache : &String)
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