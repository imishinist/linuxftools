use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Fadvise {
}

impl Fadvise {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running fadvise command...");
        Ok(())
    }
}