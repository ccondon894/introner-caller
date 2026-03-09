mod depth;
mod bed;
mod caller;
mod output;
use clap::Parser;
use crate::output::{OutputRow, SimpleOutputRow};
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(about = "Introner presence/absence caller")]
struct Args {
    bed_file: String,
    depth_file: String,
    output_file: String,

    /// Output format: "introner" (default 13-col) or "simple" (6-col input)
    #[arg(long, default_value = "introner")]
    format: String,
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();
    let depth_map = depth::load_depth_file(&args.depth_file)?;

    match args.format.as_str() {
        "introner" => {
            let bed_records = bed::load_bed_file(&args.bed_file)?;

            let results: Vec<OutputRow> = bed_records
                .par_iter()
                .filter(|record| record.presence != 3)
                .filter_map(|record| {
                    let call_result = caller::call_presence(record, &depth_map);
                    call_result.stats.map(|stats| {
                        OutputRow::new(record, &stats, call_result.call)
                    })
                })
                .collect();
            output::write_results(&results, &args.output_file)?;
        }
        "simple" => {
            let bed_records = bed::load_simple_bed_file(&args.bed_file)?;

            let results: Vec<SimpleOutputRow> = bed_records
                .par_iter()
                .filter(|record| record.presence != 3)
                .filter_map(|record| {
                    let call_result = caller::call_presence_at(
                        &record.contig, record.start, record.end, &depth_map,
                    );
                    call_result.stats.map(|stats| {
                        SimpleOutputRow::new(record, &stats, call_result.call)
                    })
                })
                .collect();
            output::write_simple_results(&results, &args.output_file)?;
        }
        other => {
            anyhow::bail!("Unknown format '{}'. Expected 'introner' or 'simple'.", other);
        }
    }

    Ok(())
}
