use alloc::{string::{String, ToString}, vec::Vec};
use spin::Mutex;

use crate::println;

#[derive(Copy, Clone, Debug)]
pub enum AvocadOSMode
{
    QEMU,
    UNSUPPORTED
}

pub struct EnvAvoOsMode
{
    pub value : AvocadOSMode
}

pub static MODE : Mutex<EnvAvoOsMode> = Mutex::new(EnvAvoOsMode {value : AvocadOSMode::UNSUPPORTED});

fn set_os_mode(var : &str)
{
    MODE.lock().value = match var
    {
        "qemu" => AvocadOSMode::QEMU,
        _ => AvocadOSMode::UNSUPPORTED
    }
}

fn get_os_mode() -> String
{
    match MODE.lock().value
    {
        AvocadOSMode::QEMU => String::from("qemu"),
        AvocadOSMode::UNSUPPORTED => String::from("unsupported host")
    }
}

fn set_variable(name : &str, var : &str)
{
    match name
    {
        "os_mode" => set_os_mode(var),
        _ => {println!("{} is an unrecognised variable", name);}
    }
}

fn get_variable(name : &str)
{
    println!("{}", match name
    {
        "os_mode" => get_os_mode(),
        _ => "unrecognised variable".to_string()
    })
}

pub fn env_cmd(input : &String)
{
    let args : Vec<&str> = input.split(" ").collect();
    if args.len() < 2 {println!("MISSING SUBCOMMAND!");return;}
    let subcommand = args[1];
    match subcommand
    {
        "set" => {
            if args.len() < 4 {println!("expected more arguments, recieved {} expected 4", args.len());return;}
            set_variable(args[2], args[3]);

        },
        "get" => {
            if args.len() < 3 {println!("expected more arguments, recieved {} expected 3", args.len());return;}
            get_variable(args[2]);
        },
        _ => {println!("{} is not a recognised subcommand", subcommand);}
    }

}