use alloc::{borrow::ToOwned, string::{String, ToString}};
use lazy_static::lazy_static;

use crate::println;

// TODO: Change this to be dynamic and not rely on vga_buffer
pub const STDOUT_SIZE : usize = 25 * 80;

lazy_static!
{
    pub static ref SHELL : AvoShell = AvoShell::new();
}

pub struct AvoShell
{
    command_cache : String,
    enabled : bool
}

impl AvoShell
{
    fn new() -> Self
    {
        AvoShell
        {
            command_cache: String::from(""),
            enabled : false
        }
    }

    pub fn consume(&mut self)
    {
        println!("{}", &self.command_cache);
    }

    pub fn key_press(&mut self, input : char)
    {
        if !self.enabled
        {
            return;
        }

        if input.to_string() == "\n"
        {
            self.consume();
        } else 
        {
            self.command_cache.push(input);
        }
    }
}