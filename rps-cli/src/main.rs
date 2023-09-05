use clap::Parser;

mod interactive_mode;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Run RPS in non-interactive mode.")]
    non_interactive: bool
}

fn main() {
    let args: Args = Args::parse();

    if args.non_interactive == true {
        println!("Non-interactive mode...COMING SOON!");
    }
    else {
        interactive_mode::run_interactive_mode();
    }
}