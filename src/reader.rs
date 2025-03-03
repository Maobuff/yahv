use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

use crate::reader_config::ReaderConfig;

pub fn read(path: &Path, rc: ReaderConfig) -> Result<(), Error> {
    let mut file = File::open(path)?;

    let size = file.metadata()?.len();
    println!("File size is {size} bytes");
    let padding = match rc.offset_in_decimal() {
        false => format!("{size:X}").len(),
        true => format!("{size}").len(),
    };

    let mut buf = vec![0_u8; 16];
    let mut offset = 0_usize;

    loop {
        let len = file.read(buf.as_mut_slice())?;
        if len == 0 {
            break;
        }

        let offset_formated = match rc.offset_in_decimal() {
            true => format!("{offset}"),
            false => match rc.upper_case() {
                true => format!("{offset:X}"),
                false => format!("{offset:x}"),
            },
        };

        for _ in 0..padding - offset_formated.len() {
            print!("0");
        }
        print!("{offset_formated}: ");

        for (index, byte) in buf[..len].iter().enumerate() {
            match rc.upper_case() {
                true => print!("{:02X}", byte),
                false => print!("{:02x}", byte),
            }
            if index % 2 == 1 {
                print!(" ");
            }
        }

        for _ in 0..16 - len {
            print!("  ");
        }
        for _ in 0..(16-len) / 2 {
            print!(" ");
        }
        if (16 - len) % 2 == 1 {
            print!(" ");
        }

        print!("| ");
        for byte in buf[..len].iter() {
            if byte.is_ascii() && !byte.is_ascii_control() {
                print!("{}", *byte as char);
            } else {
                print!(".")
            }
        }

        println!();

        offset += len;
    }

    Ok(())
}
