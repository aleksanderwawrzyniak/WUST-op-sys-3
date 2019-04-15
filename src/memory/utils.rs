pub fn split_references(references: String) -> Vec<u64> {
    references
        .trim()
        .split_whitespace()
        .filter_map(|c| c.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}
