#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-chromium", about = "SigmaOS Chromium browser manager")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List => println!("{}: no entries configured.", "sigmaos-chromium"),
        Commands::Status => println!("{}: ready.", "sigmaos-chromium"),
    }
}
