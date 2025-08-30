use clap::{CommandFactory, Parser, Subcommand};

mod e_const;
mod pi;
mod processor;

use crate::processor::Finder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Pi to the Nth Digit
    Pi(PiArgs),
    /// Generate e to the Nth Digit
    E(EArgs),
}

#[derive(clap::Args)]
struct PiArgs {
    /// Length of Pi to generate
    #[arg(short, long, default_value_t = 100)]
    len: usize,
}

#[derive(clap::Args)]
struct EArgs {
    /// Length of e (Euler constant) to generate
    #[arg(short, long, default_value_t = 100)]
    len: usize,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Pi(args)) => {
            let p = pi::Pi { len: args.len };
            let pi_string = p.sprint();
            println!("Pi ({} digits): {}", args.len, pi_string);
        }
        Some(Commands::E(args)) => {
            let e = e_const::EulerConst { len: args.len };
            let e_string = e.sprint();
            println!("e ({} digits): {}", args.len, e_string);
        }
        None => {
            // If no subcommand is provided, print the help message.
            // clap automatically generates a help message for the top-level command.
            Cli::command().print_help().unwrap();
        }
    }
}
