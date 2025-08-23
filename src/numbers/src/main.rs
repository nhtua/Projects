use clap::{Parser, Subcommand};

mod e_const;
mod pi;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    /// Length of e to generate
    #[arg(short, long, default_value_t = 100)]
    len: usize,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pi(args) => {
            let pi_string = pi::sprint(args.len as u32);
            println!("Pi ({} digits): {}", args.len, pi_string);
        }
        Commands::E(args) => {
            let e_string = e_const::sprint(args.len as u32);
            println!("e ({} digits): {}", args.len, e_string);
        }
    }
}
