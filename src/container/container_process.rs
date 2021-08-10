extern crate unshare;
extern crate libc;

use unshare::Namespace::{ Net, Uts, Pid, Mount, Ipc };
use std::cell::RefCell;


pub fn new_parent_process(tty: bool, command: &str) ->  RefCell<unshare::Command> {
    let args = ["init", "-c", command];
    let mut cmd = unshare::Command::new("/proc/self/exe");
    cmd.args(&args[..]);
    let namespaces: Vec<unshare::Namespace> = vec![Net, Uts, Pid, Mount, Ipc];
    cmd.unshare(&namespaces);

    if tty {
        cmd.stdin(unshare::Stdio::inherit());
        cmd.stdout(unshare::Stdio::inherit());
        cmd.stderr(unshare::Stdio::inherit());
    }

    return RefCell::new(cmd);
}













