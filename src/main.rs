use thesauromatic::lookup;
use toml::Value;

#[paw::main]
fn main(args: paw::Args) {
    let args = args.collect::<Vec<_>>();
    match args.get(1) {
        None => {
            // Grab the version directly out of the cargo file.
            let config_file = include_str!("../Cargo.toml");
            let config = config_file.parse::<Value>().unwrap();
            let version = &config["package"]["version"].as_str().unwrap();

            println!("Thesauromatic {}\n\nUsage: thesauromatic <word>", version);
            std::process::exit(0);
        }
        Some(w) => {
            let v = lookup(w);
            println!("{}", v.join("\n"));
            std::process::exit(0);
        }
    }
}
