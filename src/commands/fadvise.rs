use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use crate::filesystem::{self, Advise};

#[derive(Parser, Debug)]
pub struct Fadvise {
    #[clap(value_name = "filename")]
    file_name: PathBuf,
    #[clap(value_name = "mode", ignore_case = true)]
    advise: Advise,

    #[clap(short, long, value_name = "offset")]
    offset: Option<u64>,
    #[clap(short, long, value_name = "length")]
    length: Option<usize>,
}

impl Fadvise {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Going to fadvise {} as mode {}",
            self.file_name.display(),
            self.advise
        );

        let file = File::options()
            .read(true)
            .write(true)
            .open(&self.file_name)?;
        let metadata = file.metadata()?;
        let offset = self.offset.unwrap_or(0);
        let length = self.length.unwrap_or(metadata.len() as usize);
        println!("offset: {}", offset);
        println!("length: {}", length);
        println!("mode: {}", self.advise);

        filesystem::fadvise(&file, self.advise, Some(offset), Some(length))?;
        println!("WIN");
        Ok(())
    }
}
