pub fn is_permutation_small<T>(lhs: &[T], rhs: &[T]) -> bool
where
    T: PartialEq,
{
    if lhs.len() != rhs.len() {
        return false;
    }

    // Horrible O(n^2) solution, but may be faster than using a HashMap for small arrays
    // TODO: Measure performance and see if it's worth it to use a HashMap
    for x in lhs {
        let mut found = false;
        for y in rhs {
            if x == y {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

pub fn parse_kv_opt_string(s: Option<String>, num_default_entries: Option<usize>) -> Vec<Option<String>> {
    // a=1;b=2;c=3;;d=4 -> ["a=1", "b=2", "c=3", "", "d=4"]
    // "" -> [] or ["", "", "", ...] `depending on num_default_entries`
    let num_entries = num_default_entries.unwrap_or_default(); // Defaults to 0
    s
        .unwrap_or_else(|| ";".repeat(std::cmp::max(1, num_entries) - 1)) // Avoid underflow issues
        .split(';')
        .map(|s| {
            if s.is_empty() {
                return None
            }
            Some(s.to_owned())
        })
        .collect()
}