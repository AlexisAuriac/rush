pub fn split_no_empty(line: &String) -> Vec<String> {
    line.split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.trim() != "")
        .collect()
}
