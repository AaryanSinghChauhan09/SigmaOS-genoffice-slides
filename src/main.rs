#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "genoffice-slides", about = "SigmaOS GenOffice slides")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("genoffice-slides: no entries configured."),
        Commands::Status => println!("genoffice-slides: ready."),
    }
}
