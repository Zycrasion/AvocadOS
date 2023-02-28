use alloc::{string::{String}};
use lazy_static::lazy_static;
use pc_keyboard::KeyCode;

mod echo;
mod shutdown;

use crate::{println, allocator::Locked, print, vga_buffer::{WRITER, ColorCode, Color}, avo_shell::{echo::echo, shutdown::shutdown_cmd}};


lazy_static!
{
    pub static ref SHELL : Locked<AvoShell> = Locked::<AvoShell>::new(AvoShell::new());
}

pub struct AvoShell
{
    command_cache : String,
    shell_start : String,
    enabled : bool
}

impl AvoShell
{
    pub fn new() -> Self
    {
        return AvoShell { command_cache: String::from(""), enabled: false, shell_start: String::from(">") }
    }

    pub fn consume(&mut self)
    {
        if &self.command_cache == ""
        {
            return;
        }

        let mut args = self.command_cache.split_whitespace();
        let command = args.next().unwrap_or("");

        if command == "" {return;}
        
        println!();

        match command
        {
            "echo" => {let _ = echo(&self.command_cache);},
            "shutdown" => {shutdown_cmd(&self.command_cache);}
            _ => {println!("command not recognised!");}
        }

        self.command_cache = String::from("");
    }

    pub fn on_shell_enable(&mut self)
    {
        self.enabled = true;
        WRITER.lock().color_code = ColorCode::new(Color::LightGreen, Color::Black);
        println!("Welcome to the kernel debugger (AvoShell)");
        self.on_newline();
    }

    pub fn on_newline(&mut self)
    {
        self.consume();
        self.update_line();
    }

    pub fn on_backspace(&mut self)
    {
        self.command_cache.pop();
        self.update_line();
    }

    pub fn update_line(&mut self)
    {
        WRITER.lock().clear_current_line();
        print!("{} {}", self.shell_start,self.command_cache)
    }

    pub fn on_key(&mut self, input : &char)
    {
        self.command_cache.push(*input);
        self.update_line();
    }
}

impl Locked<AvoShell>
{
    pub fn key_press(&self, input : char)
    {
        let mut shell = self.lock();
        if input == '~' && shell.enabled == false {shell.on_shell_enable(); return;}

        if !shell.enabled
        {
            return;
        }
        
        match input
        {
            
            '\n' => shell.on_newline(),
            '\u{008}' => shell.on_backspace(),
            _ => shell.on_key(&input)
        }
    }

    // pub fn raw_key_press(&self, input : KeyCode)
    // {
    //     match input
    //     {

    //     }
    // }
}