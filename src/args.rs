use std::path::PathBuf;
use clap::{Subcommand, Args, AppSettings};

#[derive(Subcommand)]
pub enum MainArgs {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode(EncodingArgs),

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode(DecodingArgs),

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Remove(RemovingArgs)

}

#[derive(Args, Debug)]
pub struct EncodingArgs {
    pub path: PathBuf,

    pub chunk_type: String,

    pub message: String,

    pub key: Option<String>,

    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodingArgs {
    pub path: PathBuf,

    pub chunk_type: String,

    pub key: Option<String>,
}

#[derive(Args,Debug)]
pub struct RemovingArgs {
    pub path: PathBuf,

    pub chunk_type: String,

    pub key: Option<String>,

    pub output: Option<PathBuf>,
}
