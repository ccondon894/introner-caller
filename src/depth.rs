use std::collections::HashMap;

pub type DepthMap = HashMap<String, HashMap<u32, u32>>;

// What we need for coverage stats:
// mean_depth for left, right and middle regions 
// coverage fraction for middle region (% bases with coverage)
// has_continuous_junctions checks for dropouts around the introner junctions
#[derive(Debug)]

pub struct CoverageStats {
    pub coverage_fraction: f64,
    pub mean_depth_introner: f64,
    pub mean_depth_left: f64,
    pub mean_depth_right: f64,
    pub has_continuous_junctions: bool,

}



pub fn load_depth_file(path: &str) -> anyhow::Result<DepthMap> {
    let mut depth_map = DepthMap::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path)?;
    for result in reader.records() {
        let record = result?;
        let chrom = record[0].to_string();
        let pos = record[1].parse::<u32>()?;
        let depth = record[2].parse::<u32>()?;

        depth_map
            .entry(chrom) // check if outer key exists 
            .or_insert_with(HashMap::new) // create with a new empty hashmap if doesn't exist 
            .insert(pos, depth); // insert the pos + depth 

    }
    Ok(depth_map)
}

pub fn get_region_coverage(depth_map: &DepthMap, chrom: &str, start: u32, end: u32) -> Option<CoverageStats> {
    let introner_range: u32 = (end - 100) - (start + 100);
    let mut depth_sum_left: u32 = 0;
    let mut depth_sum_right: u32 = 0;
    let mut depth_sum_introner: u32 = 0;
    let mut is_continuous: bool = true;
    let mut coverage: u32 = 0;
    let chrom_map = depth_map.get(chrom)?;
    let left_junction = (start + 90)..(start + 110);
    let right_junction = (end - 110)..(end - 90);

    for pos in start..end {

        let depth = chrom_map
            .get(&pos)
            .copied()
            .unwrap_or(0);

        if depth == 0 && (left_junction.contains(&pos) || right_junction.contains(&pos)) {
            is_continuous = false;
        }

        if pos < start + 100 {
            depth_sum_left += depth;

        } else if pos >= start + 100 && pos < end - 100 {
            depth_sum_introner += depth;
            if depth != 0 {
                coverage += 1;
            }

        } else {
            depth_sum_right += depth;
        }

    }  
    Some(CoverageStats {
        mean_depth_introner: depth_sum_introner as f64 / introner_range as f64,
        mean_depth_left: depth_sum_left as f64 / 100.0,
        mean_depth_right: depth_sum_right as f64 / 100.0,
        coverage_fraction: coverage as f64 / introner_range as f64,
        has_continuous_junctions: is_continuous,
    })
}