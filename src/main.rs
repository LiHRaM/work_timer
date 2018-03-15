#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "The Work Timer", about="A time-tracking tool for people who do stuff.")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name="punch-in",
                about="Register the beginning of a work-period.")]
    PunchIn,
    #[structopt(name="punch-out",
                about="Register the end of a work-period.")]
    PunchOut
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt.cmd {
        Some(Command::PunchOut) => {
            println!("Punching out...");
        },
        Some(Command::PunchIn) => {
            println!("Punching in...");
        },
        _ => println!("What are you doing, man!?")
    }
}
