use clap::Parser;

#[derive(Parser, Debug)]
pub struct Fincore {
}

impl Fincore {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running fincore command...");
        Ok(())
    }
}