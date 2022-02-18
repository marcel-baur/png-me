mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::{Parser, AppSettings};
use args::MainArgs;

/// Program to encode messages in png files
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Program {
    #[clap(subcommand)]
    command: MainArgs,
}


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = Program::parse();

    match &args.command {
        MainArgs::Encode(args) => {
            commands::encode(&args);
            
        }
        MainArgs::Decode(args) => {
            todo!();
        }
    }


}
