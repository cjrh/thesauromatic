use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let syns = BufReader::new(File::open("mobylf.txt").unwrap());
    let mut m = phf_codegen::Map::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();

    syns.lines()
        // .take(10)
        .for_each(|line| {
            let line = line.unwrap();
            let v: Vec<&str> = line.splitn(2, ',').collect();
            let key = v.get(0).unwrap().to_string();
            let value = v.get(1).unwrap().to_string();
            if !seen.contains(&key) {
                m.entry(key.clone(), format!("\"{}\"", &value).as_str());
                seen.insert(key);
            }
        });

    let mut file = BufWriter::new(File::create(&path).unwrap());
    writeln!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, &'static str> = \n{};\n",
        m.build()
    )
    .unwrap();
}
