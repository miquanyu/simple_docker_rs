mod command;
mod utils;
mod container;
mod run;

use structopt::StructOpt;
use command::Opt;

fn main() {
    let matches = Opt::from_args();
    match matches {
        Opt::Run{
            tty,
            detach: _,
            memory: _,
            cpu_share: _,
            cpu_set: _,
            volume: _,
            name: _,
            env: _,
            port: _,
            command,
        } => {
            run::run(tty, command.as_str());
        }
        Opt::Init {
            command,
        } => {
            let args: Vec<String> = Vec::new();
            match container::run_container_init_process(command, &args) {
                Ok(()) => (),
                Err(e) => println!("{:?}", e),
            };
        }
    }
}
