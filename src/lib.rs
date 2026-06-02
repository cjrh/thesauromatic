include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Return the related words for `word` as a borrowed, comma-separated slice
/// of the embedded data, or `None` if the word is unknown.
///
/// This is the zero-allocation core of the lookup: the value points directly
/// into the statically linked word table. Callers that need owned, split
/// strings should use [`lookup`]; callers that only stream the words (e.g. the
/// CLI) can split this slice themselves and avoid all allocation.
pub fn lookup_raw(word: &str) -> Option<&'static str> {
    KEYWORDS.get(word).copied()
}

/// Return the related words for `word`, one owned `String` per word.
///
/// Returns an empty vector if the word is unknown.
pub fn lookup(word: &str) -> Vec<String> {
    match lookup_raw(word) {
        None => vec![],
        Some(synonyms) => synonyms.split(',').map(|s| s.to_owned()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        let out = lookup("happy");
        println!("happy: {:?}", out);
        assert!(out.contains(&"to be desired".to_string()));
    }

    #[test]
    fn missing_word() {
        let out = lookup("blahblah");
        println!("happy: {:?}", out);
        assert_eq!(out.len(), 0);
    }
}
