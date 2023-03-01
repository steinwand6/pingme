use std::str::Bytes;

/*
mod args;
mod commands;
mod png;
*/
mod chunk;
mod chunk_type;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    todo!()
}
