pub fn parse_number_list(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .into_iter()
        .map(|x| x.parse())
        .flatten()
        .collect()
}
