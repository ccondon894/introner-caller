# introner-caller

A tool for calling introner presence/absence from sequencing depth data.

## Overview

introner-caller takes a BED file of introner annotations and a per-base depth file, then re-evaluates each introner's presence or absence based on coverage statistics. It outputs a TSV with the original call alongside a new call derived from depth evidence.


## Usage

```
introner-caller <BED_FILE> <DEPTH_FILE> <OUTPUT_FILE>
```

- **BED_FILE** - Tab-delimited file with columns: contig, start, end, sample, ortholog_id, sequence_id, family, gene, splice_site, presence, orientation, left_reverse, right_reverse
- **DEPTH_FILE** - Tab-delimited, headerless, three-column file (chrom, position, depth) as produced by `samtools depth`
- **OUTPUT_FILE** - Path for the output TSV

## Building

```
cargo build --release
```

