use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
