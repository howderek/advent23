pub fn parse_number_list(s: &str) -> Vec<u64> {
    s.split(" ")
        .into_iter()
        .map(|x| x.parse())
        .flatten()
        .collect()
}
