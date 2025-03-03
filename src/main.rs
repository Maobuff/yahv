use std::{env::args, path::Path};

use reader_config::ReaderConfig;

mod reader;
mod reader_config;

fn usage() {
    println!("Usage:");
    println!("\t yahv [options] file");
    println!("Options:");
    println!("\t-u\tuse upper case hex letters.");
    println!("\t-d\tshow offest in decimal.");
}

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    let path = match args.last() {
        Some(v) => v.clone(),
        None => {
            usage();
            return;
        }
    };

    let path = Path::new(&path);
    if !path.exists() {
        usage();
        return;
    }

    match ReaderConfig::try_from(args) {
        Ok(rc) => {
            println!("Reading file: {}", path.to_str().unwrap());
            if let Err(e) = reader::read(path, rc) {
                println!("{e}");
            }
        }
        Err(e) => println!("{e}"),
    }
}
