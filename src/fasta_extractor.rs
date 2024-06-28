use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use rayon::prelude::*;

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about = "Extract FASTA headers from a file.", long_about = None)]
struct Args {
    /// Path to the input FASTA file (required)
    #[arg(required = true)]
    input: String,

    /// Number of parallel jobs (default: all available CPUs)
    #[arg(short, long)]
    njobs: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let opts: Args = Args::parse();

    let num_jobs = opts.njobs.unwrap_or_else(num_cpus::get);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build()?;

    let file = File::open(&opts.input)?;
    let reader = BufReader::new(file);

    // Print status message to stderr
    eprintln!(
        "Extracting headers from FASTA file... using {} jobs",
        num_jobs
    );

    pool.install(|| {
        reader
            .lines()
            .par_bridge()
            .filter_map(Result::ok)
            .filter(|line| line.starts_with('>'))
            .for_each(|header| {
                println!("{}", header);
            });
    });

    Ok(())
}
