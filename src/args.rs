use std::path::PathBuf;
use clap::{Subcommand, Args, AppSettings};

#[derive(Subcommand)]
pub enum MainArgs {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode(EncodingArgs),

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode(DecodingArgs)
}

#[derive(Args, Debug)]
pub struct EncodingArgs {
    #[clap(parse(from_os_str), short, long)]
    pub path: PathBuf,

    pub chunk_type: String,

    pub message: String,

    pub key: Option<String>,

    pub output: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodingArgs {
    #[clap(parse(from_os_str), short='i', long)]
    pub path: PathBuf,

    pub chunk_type: String,

    #[clap(short, long)]
    pub key: Option<String>,
}