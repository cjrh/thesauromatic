// 123

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn lookup(word: &str) -> Vec<String> {
    KEYWORDS.get(word)
        .or(Some(&""))
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn happy() {
        let out = lookup("happy");
        println!("happy: {:?}", out);
        assert!(out.contains(&"to be desired".to_string()));
    }
}
