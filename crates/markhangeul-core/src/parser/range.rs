pub const RANGE_OPEN: &str = "((";
pub const RANGE_CLOSE: &str = "))";

pub fn find_closing_brace(source: &str, open_index: usize) -> Option<usize> {
    source[open_index + 1..]
        .find('}')
        .map(|relative_index| open_index + 1 + relative_index)
}
