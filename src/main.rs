use std::{env::args, path::Path};

use reader_config::ReaderConfig;

mod reader;
mod reader_config;

fn usage() {
    println!("Usage:");
    println!("\t yahv [options] file");
    println!("Options:");
    println!("\t-u\t\tuse upper case hex letters.");
    println!("\t-d\t\tshow offest in decimal.");
    println!("\t-g bytes\tnumber of octets per group. Default 2.");
    println!("\t-h\t\tprint usage.");
    println!("\t-s seek\t\tstart at <seek>.");
    println!("\t-c cols\t\t<cols> octets per line. Default 16.");
    println!("\t-l len\t\tstop after <len> octets.");
}

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.contains(&String::from("-h")) {
        usage();
        return;
    }

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
