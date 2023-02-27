use core::str::SplitWhitespace;

use alloc::string::String;

use crate::{print, println};

pub fn echo(command_cache : &String) -> Result<(), ()>
{
    let mut args = command_cache.split_whitespace();
    args.next(); // SKIP ECHO
    for arg in args
    {
        print!("{} ", arg)
    }

    println!();

    Ok(())
}