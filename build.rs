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

    link_binary_without_pie();
}

/// Link the CLI binary as a non-PIE executable on Linux.
///
/// The embedded word table is a `phf::Map` of ~30k `&'static str` keys and
/// values. In a position-independent executable, every one of those ~60k
/// pointers becomes a load-time relocation that the dynamic linker must apply
/// before `main` runs — which dominated the program's startup cost. Linking
/// the binary as non-PIE lets the linker bake those addresses in at link time,
/// removing essentially all of the relocations and bringing startup down to
/// the process-spawn floor.
///
/// `rustc-link-arg-bins` scopes this to binary targets only, so the `pycrate`
/// `cdylib` (which must stay position-independent) is unaffected. `-no-pie` is
/// a GNU/ELF linker flag, so it is emitted only for Linux targets; macOS and
/// Windows keep their default linking.
fn link_binary_without_pie() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-arg-bins=-no-pie");
    }
}
