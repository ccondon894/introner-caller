use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BedRecord {
    pub contig: String,
    pub start: u32,
    pub end: u32,
    pub sample: String,
    pub ortholog_id: String,
    pub sequence_id: String,
    pub family: String,
    pub gene: String,
    pub splice_site: String,
    pub presence: u8,
    pub orientation: String,
    pub left_reverse: String,
    pub right_reverse: String,
}

#[derive(Debug, Deserialize)]
pub struct SimpleBedRecord {
    pub contig: String,
    pub start: u32,
    pub end: u32,
    pub intron_id: String,
    pub gene: String,
    pub presence: u8,
}

pub fn load_bed_file(path: &str) -> anyhow::Result<Vec<BedRecord>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)?;

    let records = reader.deserialize()
        .collect::<Result<Vec<BedRecord>, _>>()?;
    Ok(records)
}

pub fn load_simple_bed_file(path: &str) -> anyhow::Result<Vec<SimpleBedRecord>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)?;

    let records = reader.deserialize()
        .collect::<Result<Vec<SimpleBedRecord>, _>>()?;
    Ok(records)
}
