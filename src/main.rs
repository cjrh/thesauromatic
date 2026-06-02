use std::io::{self, Write};
use thesauromatic::lookup_raw;

fn main() {
    // Lock stdout once and write through a single buffered handle. The output
    // is tiny (a few KB at most), so one buffered pass costs one write syscall.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    match std::env::args().nth(1) {
        None => {
            let _ = writeln!(
                out,
                "Thesauromatic {}\n\nUsage: thesauromatic <word>",
                env!("CARGO_PKG_VERSION")
            );
        }
        Some(word) => {
            // The data stores related words comma-separated; emit one per line.
            // Writing the borrowed slice piece-by-piece avoids allocating an
            // owned String per word. An unknown word still emits a single blank
            // line, matching the long-standing output contract.
            if let Some(words) = lookup_raw(&word) {
                for w in words.split(',') {
                    let _ = out.write_all(w.as_bytes());
                    let _ = out.write_all(b"\n");
                }
            } else {
                let _ = out.write_all(b"\n");
            }
        }
    }
}
