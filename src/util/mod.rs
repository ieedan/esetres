/// Returns a right padded string based on the min length
pub fn min_length(str: &str, min: usize) -> String {
    let mut str = String::from(str);

    let remaining = min - str.len();

    for _ in 0..remaining {
        str.push_str(" ");
    }

    str
}

/// Returns a string with characters provided times the amount specified
pub fn repeat(char: &str, amount: usize) -> String {
    let mut str = String::new();
    for _ in 0..amount {
        str.push_str(char);
    }

    str
}