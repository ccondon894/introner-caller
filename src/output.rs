use crate::{caller::Call, depth::CoverageStats};
use crate::bed::BedRecord;
use std::io::{BufWriter, Write};
use std::fs::File;

#[derive(Debug)]
pub struct OutputRow {
    pub chrom: String,
    pub start: u32,
    pub end: u32,
    pub sample: String,
    pub ortholog_id: String,
    pub sequence_id: String,
    pub family: String,
    pub gene: String,
    pub splice_site: String,
    pub orientation: String,
    pub left_reverse: String,
    pub right_reverse: String,
    pub depth_left: f64,
    pub depth_introner: f64,
    pub depth_right: f64,
    pub old_call: Call,
    pub new_call: Call,
}

impl OutputRow {
    pub fn new(record: &BedRecord, stats: &CoverageStats, new_call: Call) -> Self {
        OutputRow {
            chrom: record.contig.clone(),
            start: record.start,
            end: record.end,
            sample: record.sample.clone(),
            ortholog_id: record.ortholog_id.clone(),
            sequence_id: record.sequence_id.clone(),
            family: record.family.clone(),
            gene: record.gene.clone(),
            splice_site: record.splice_site.clone(),
            orientation: record.orientation.clone(),
            left_reverse: record.left_reverse.clone(),
            right_reverse: record.right_reverse.clone(),
            depth_left: stats.mean_depth_left,
            depth_introner: stats.mean_depth_introner,
            depth_right: stats.mean_depth_right,
            old_call: Call::from(record.presence),
            new_call,
        }
    }
}

pub fn write_results(results: &[OutputRow], path: &str) -> anyhow::Result<()> {
    let header_line = "contig\tstart\tend\tsample\tortholog_id\tsequence_id\tfamily\tgene\tsplice_site\torientation\tleft_reverse\tright_reverse\tleft_mean_depth\tmean_middle_depth\tright_mean_depth\tcall\tnew_call";
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer,"{}", header_line)?;
    for row in results {
        writeln!(writer, 
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        row.chrom, row.start, row.end, row.sample, row.ortholog_id, row.sequence_id,
        row.family, row.gene, row.splice_site, row.orientation, row.left_reverse,
        row.right_reverse, row.depth_left, row.depth_introner, row.depth_right,
        row.old_call, row.new_call)?;
    }
    Ok(())
}