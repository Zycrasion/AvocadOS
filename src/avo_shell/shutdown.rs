

use alloc::string::String;


use crate::{println, exit_qemu, QemuExitCode, acpi::shutdown};


pub fn shutdown_cmd(_command_cache : &String)
{
    shutdown();
}