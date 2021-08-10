mod container_process;

use nix::unistd::execv;

pub use container_process::new_parent_process;
use crate::utils::{ parse_one_string_to_cstr, parse_more_string_to_cstr };
use std::ffi::CStr;
use nix::errno::Errno;


pub fn run_container_init_process(command: String, args: &[String]) -> Result<(), Errno> {
    println!("command {}", command);
    let cmd= parse_one_string_to_cstr(&command);
    let mut args_cstr: Vec<&CStr> = Vec::new();
    if args.len() > 0 {
        args_cstr = parse_more_string_to_cstr(args);
    }
    execv(cmd, &args_cstr)?;
    Ok(())
}

