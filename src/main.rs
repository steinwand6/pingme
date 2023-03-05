use args::PngMeArgs;
use clap::Parser;
use commands::{decode, encode, print, remove, PngMeCommmands};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = PngMeCommmands::parse();
    match cli.action {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    }
}
