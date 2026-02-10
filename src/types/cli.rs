use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Specify the directory
    #[arg(short, long)]
    pub dir: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Subcommand)]
pub enum Commands {}
