include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn lookup(word: &str) -> Vec<String> {
    match KEYWORDS.get(word) {
        None => vec![],
        Some(synonyms) => {
            synonyms.split(',').map(|s| s.to_owned()).collect()
        }
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
