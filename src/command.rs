use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "simple_docker")]
pub enum Opt {
    #[structopt(help = "Create a container with namespace and cgroups limit mydocker run - t i [command ]")]
    Run {
        #[structopt(short = "t", help = "enable tty")]
        tty: bool,
        #[structopt(short = "d", help = "detach container")]
        detach: bool,
        #[structopt(short = "m", required = false, help = "memory limit", default_value="10")]
        memory: String,
        #[structopt(long = "cpushare", required = false, help = "cpu share limit", default_value="1")]
        cpu_share: String,
        #[structopt(long = "cpuset", required = false, help = "cpu set limit", default_value="1")]
        cpu_set: String,
        #[structopt(short = "v", required = false, help = "volume", default_value="10")]
        volume: String,
        #[structopt(short = "n", required = false, help = "container name", default_value="")]
        name: String,
        #[structopt(short = "e", required = false, help = "set environment", default_value="")]
        env: String,
        #[structopt(short = "p", required = false, help = "port mapping", default_value="")]
        port: String,
        #[structopt(short = "c", required = false, help = "command", default_value="")]
        command: String,
    },
    Init {
        #[structopt(short = "c", required = false, help = "command", default_value="")]
        command: String,
    },
}