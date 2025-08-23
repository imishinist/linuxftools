use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use crate::filesystem;

#[derive(Parser, Debug)]
pub struct Fallocate {
    #[clap(value_name = "file")]
    file_name: PathBuf,

    #[clap(value_name = "length")]
    length: usize,
}

impl Fallocate {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Going to fallocate {}", self.file_name.display());

        let file = File::options()
            .read(true)
            .write(true)
            .open(&self.file_name)?;

        log_stats(&file)?;

        println!("Increasing file to {}", self.length);
        filesystem::fallocate(&file, self.length)?;

        log_stats(&file)?;
        Ok(())
    }
}

fn log_stats(file: &File) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::MetadataExt;

    let metadata = file.metadata()?;

    println!("File stats: ");
    println!("    Length:           {}", metadata.len());
    println!("    Block size:       {}", metadata.blksize());
    println!("    Blocks allocated: {}", metadata.blocks());
    Ok(())
}
