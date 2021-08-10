use crate::container::new_parent_process;
use std::process::exit;

pub fn run(tty: bool, command: &str) {
    let parent = new_parent_process(tty, command);
    let process = &parent;
    let mut child = match process.borrow_mut().spawn() {
        Ok(child) => { child },
        Err(e) => {
            println!("{:?}", e);
            exit(127);
        }
    };
    child.wait().expect("parent process wait failed");
    exit(-1);
}