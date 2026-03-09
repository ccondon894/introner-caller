
// caller steps:
// retrieve coverage for left, middle and right regions
// check if flanking region mean depth > 10
// check introner coverage fraction > 0.8
// check for continuous junctions
// check introner mean depth > 10
// check unbalanced depth differences: introner_mean_depth > 3 * left/right
// skip uniformity
// skip model_coverage_pattern
use crate::bed::BedRecord;
use crate::depth::{DepthMap, CoverageStats, get_region_coverage};
use std::fmt;


// derive PartialEq trait since we want to use == to see if call changed
#[derive(Debug, PartialEq, Clone)]
pub enum Call {
    Present,
    Absent,
    Missing,
}

pub struct CallResult {
    pub call: Call,
    pub stats: Option<CoverageStats>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Present -> "1", Absent -> "2", Missing -> "3"
        let output = match self {
            Call::Present => "1",
            Call::Absent => "2",
            Call::Missing => "3",
        };
        write!(f, "{}", output)
    }
}

impl From<u8> for Call {
    fn from(val: u8) -> Self {
        match val {
            1 => Call::Present,
            2 => Call::Absent,
            3 => Call::Missing,
            _ => Call::Missing, // treat unexpected variables as uncallable
        }
    }
}

pub fn call_presence_at(chrom: &str, start: u32, end: u32, depth_map: &DepthMap) -> CallResult {
    let stats: CoverageStats = match get_region_coverage(depth_map, chrom, start, end) {
        None => return CallResult { call: Call::Missing, stats: None },
        Some(s) => s,
    };
    // implement caller logic here
    if stats.mean_depth_left < 10.0 || stats.mean_depth_right < 10.0 {
        return CallResult { call: Call::Missing, stats: Some(stats) }
    }
    if stats.coverage_fraction < 0.8 {
        return CallResult {call: Call::Absent, stats: Some(stats)}
    }
    if stats.mean_depth_introner < 10.0 {
        return CallResult {call: Call::Absent, stats: Some(stats)}
    }
    if !stats.has_continuous_junctions {
        return CallResult {call: Call::Absent, stats: Some(stats)}
    }
    if stats.mean_depth_introner > 3.0 * stats.mean_depth_left.max(stats.mean_depth_right) {
        return CallResult {call: Call::Absent, stats: Some(stats)}
    }

    CallResult {call: Call::Present, stats: Some(stats)}
}

pub fn call_presence(bed_record: &BedRecord, depth_map: &DepthMap) -> CallResult {
    call_presence_at(&bed_record.contig, bed_record.start, bed_record.end, depth_map)
}