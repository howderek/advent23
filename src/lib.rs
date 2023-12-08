pub mod vendor;

pub fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .into_iter()
        .map(|x| x.parse())
        .flatten()
        .collect()
}
