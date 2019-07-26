/// Splits `item` into an vector of characters.
///
/// # Arguments
///
/// * `item` - The string to split into characters.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// split::chars("cloud");
/// // => ["c", "l", "o", "u", "d"]
/// ```
pub fn chars(item: &str) -> Vec<&str> {
    if item.is_empty() {
        return vec![""];
    }
    // The split() function returns an array of chars, adding a space
    // at the beginning,
    // skip the first space:
    item.split_terminator("").skip(1).collect::<Vec<_>>()
}


pub fn split<'a>(item: &'a str, pattern: &str) -> Vec<&'a str> {
    // If the item is empty, return an empty vector.  
    // If pattern is empty, return a vector of the item
    if item.is_empty() { return vec![""];  }
    if pattern.is_empty() { return vec![item]; }

    // create a vector split by the pattern (removes separator)
    item.split_terminator(pattern).collect::<Vec<_>>()
}