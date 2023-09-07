use clap::Parser;

mod interactive_mode;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct RPSArgs {
    #[arg(short, long, default_value_t = false, help = "Run RPS in non-interactive mode.")]
    non_interactive: bool,
    #[arg(short, long, default_value_t = false, help = "Print log messages in terminal window.")]
    verbose: bool,
    #[arg(short, long, default_value_t = 3, help = "Number of wins a player needs to win the series.")]
    win_count: usize,
}

fn main() {
    let args: RPSArgs = RPSArgs::parse();

    if args.non_interactive == true {
        println!("Non-interactive mode...COMING SOON!");
    }
    else {
        interactive_mode::run_interactive_mode(&args);
    }
}