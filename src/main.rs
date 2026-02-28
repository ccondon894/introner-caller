mod depth;
mod bed;
mod caller;
mod output;
use clap::Parser;
use crate::output::OutputRow;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(about = "Introner presence/absence caller")]
struct Args {
    bed_file: String,
    depth_file: String,
    output_file: String,

}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();
    let depth_map = depth::load_depth_file(&args.depth_file)?;
    let bed_records  = bed::load_bed_file(&args.bed_file)?;

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
    Ok(())
}
