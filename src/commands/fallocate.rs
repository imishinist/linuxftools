use clap::Parser;

#[derive(Parser, Debug)]
pub struct Fallocate {
}

impl Fallocate {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running fallocate command...");
        Ok(())
    }
}