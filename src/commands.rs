use clap::Parser;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use crate::args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;

#[derive(Parser)]
pub struct PngMeCommmands {
    #[clap(subcommand)]
    pub action: PngMeArgs,
}

pub fn encode(args: EncodeArgs) -> Result<(), Box<dyn Error>> {
    let input = fs::read(&args.file_path)?;
    let output = args.output_file.unwrap_or(args.file_path);
    let mut png = Png::try_from(input.as_slice())?;
    let chunk = Chunk::new(args.chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);
    let mut output = File::create(output)?;
    output.write(png.as_bytes().as_slice())?;
    println!("success!");
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<(), Box<dyn Error>> {
    let input = fs::read(&args.file_path)?;
    let png = Png::try_from(input.as_slice())?;
    let res = png.chunk_by_type(args.chunk_type.to_string().as_str());
    if let Some(chunk) = res {
        println!("{chunk}");
    } else {
        println!("chunk type {} is not found.", args.chunk_type.to_string());
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<(), Box<dyn Error>> {
    let input = fs::read(&args.file_path)?;
    let mut png = Png::try_from(input.as_slice())?;
    png.remove_chunk(args.chunk_type.to_string().as_str())?;
    let mut output = File::create(args.file_path)?;
    output.write(&png.as_bytes())?;
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<(), Box<dyn Error>> {
    let input = fs::read(&args.file_path)?;
    let png = Png::try_from(input.as_slice())?;
    png.chunks()
        .iter()
        .for_each(|c| println!("{}", c.chunk_type().to_string()));
    Ok(())
}
