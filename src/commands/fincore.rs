use std::fs::File;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::filesystem;

const DEFAULT_NR_REGIONS: usize = 160;

#[derive(Parser, Debug)]
pub struct Fincore {
    #[clap(
        short,
        long,
        help = "When comparing multiple files, print a summary report"
    )]
    summarize: bool,

    #[clap(short, long, help = "Print pages that are cached")]
    pages: bool,

    #[clap(
        short = 'o',
        long,
        help = "Only print stats for files that are actually in cache."
    )]
    only_cached: bool,

    #[clap(
        short = 'g',
        long,
        help = "Print a visual graph of each file's cached page distribution."
    )]
    graph: bool,

    #[clap(
        short = 'S',
        long,
        help = "Require that each files size be larger than N bytes."
    )]
    min_size: Option<u64>,

    #[clap(
        short = 'C',
        long,
        help = "Require that each files cached size be larger than N bytes."
    )]
    min_cached_size: Option<u64>,

    #[clap(
        short = 'P',
        long,
        help = "Require percentage of a file that must be cached."
    )]
    min_perc_cached: Option<f64>,

    #[clap(
        short = 'L',
        long,
        help = "Print the output of this script vertically."
    )]
    vertical: bool,

    #[clap(short = 'v', long)]
    verbose: bool,

    files: Vec<PathBuf>,
}

impl Fincore {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.verbose {
            println!("Running with arguments: ");
            println!("    pages: {}", self.pages);
            println!("    summarize: {}", self.summarize);
            println!("    only cached: {}", self.only_cached);
            println!("    graph: {}", self.graph);
            println!("    min size: {:?}", self.min_size);
            println!("    min cached size: {:?}", self.min_cached_size);
            println!("    min perc cached: {:?}", self.min_perc_cached);
            println!("    vertical: {}", self.vertical);
        }

        if !self.graph {
            show_headers();
        }

        let mut total_cached_bytes = 0u64;
        for file in &self.files {
            let cached_bytes = self.fincore(file);
            total_cached_bytes += cached_bytes;
        }

        if self.summarize {
            println!("---");
            println!("total cached size: {}", total_cached_bytes);
        }
        Ok(())
    }

    fn fincore<P: AsRef<Path>>(&self, path: P) -> u64 {
        let nr_regions = match termsize::get() {
            Some(size) => ((size.cols / 2) * 2 - 10) as usize,
            None => DEFAULT_NR_REGIONS,
        };

        let file = match File::options().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprint!("Could not open file: {}", path.as_ref().display());
                eprintln!(": {}", e);
                return 0;
            }
        };

        let metadata = match file.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprint!("Could not stat file: {}", path.as_ref().display());
                eprintln!(": {}", e);
                return 0;
            }
        };
        if metadata.len() == 0 {
            return 0;
        }

        let stats = match filesystem::fincore(&file) {
            Ok(s) => s,
            Err(e) => {
                eprint!("Could not fincore file: {}", path.as_ref().display());
                eprintln!(": {}", e);
                return 0;
            }
        };

        let region_ptr = (stats.total_pages - 1) / nr_regions as u64;
        let cached_perc = stats.cached_pages as f64 / stats.total_pages as f64 * 100.0;

        let mut printed = false;
        let mut regions = vec![0; nr_regions];
        assert_eq!(stats.total_pages as usize, stats.pages.len());

        for (i, page) in stats.pages.iter().enumerate() {
            if (*page & 1) == 1 {
                if self.pages {
                    print!("{} ", i);
                    printed = true;
                }

                if region_ptr > 0 {
                    let region = i / region_ptr as usize;
                    regions[region.min(nr_regions - 1)] += 1;
                }
            }
        }
        if printed {
            println!();
        }

        if let Some(min_size) = self.min_size
            && stats.file_bytes <= min_size
        {
            return 0;
        }
        if let Some(min_cached_size) = self.min_cached_size
            && stats.cached_bytes <= min_cached_size
        {
            return 0;
        }
        if let Some(min_perc_cached) = self.min_perc_cached
            && cached_perc < min_perc_cached
        {
            return 0;
        }
        if self.only_cached && stats.cached_bytes == 0 {
            return 0;
        }

        if self.graph {
            show_headers();
        }

        if self.vertical {
            println!("{}", path.as_ref().display());
            println!("size: {}", stats.file_bytes);
            println!("total pages: {}", stats.total_pages);
            println!("cached: {}", stats.cached_pages);
            println!("cached_size: {}", stats.cached_bytes);
            println!("cached_perc: {:.2}%", cached_perc);
        } else {
            println!(
                "{:<80} {:>18} {:>18} {:>18} {:>18} {:>18.2}",
                path.as_ref().display(),
                stats.file_bytes,
                stats.total_pages,
                stats.cached_pages,
                stats.cached_bytes,
                cached_perc
            );
        }

        if self.graph && stats.total_pages > nr_regions as u64 {
            let region_percs: Vec<_> = regions
                .into_iter()
                .map(|r| match r {
                    0 => 0f64,
                    _ => r as f64 / region_ptr as f64 * 100.0f64,
                })
                .collect();
            self.graph(&region_percs);
        }

        stats.cached_bytes
    }

    fn graph_header(&self, nr_regions: usize) {
        println!(" ------{}", "-".repeat(nr_regions));
    }

    fn graph(&self, regions: &[f64]) {
        println!();
        self.graph_header(regions.len());

        let perc_width = 10;
        for i in 0..perc_width {
            let perc_index = (100 - (i + 1) * perc_width) as f64;
            print!("{:>4} % ", perc_index + perc_width as f64);

            for region in regions {
                if *region < 1.0f64 {
                    print!(" ");
                } else if *region >= perc_index {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!(" | ");
        }

        self.graph_header(regions.len());
        println!();
    }
}

fn show_headers() {
    println!(
        "{:<80} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "filename", "size", "total_pages", "cached_pages", "cached_size", "cached_perc"
    );
    println!(
        "{:<80} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "--------", "----", "-----------", "------------", "-----------", "-----------"
    );
}
