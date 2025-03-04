use std::{io::{Error, ErrorKind}, slice::Iter};

#[derive(Debug)]
pub struct ReaderConfig {
    upper_case: bool,
    offset_in_decimal: bool,
    group: usize,
    seek: usize,
    cols: usize,
    len: usize,
    bin: bool,
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            upper_case: false,
            offset_in_decimal: false,
            group: 2,
            seek: 0,
            cols: 16,
            len: 0,
            bin: false,
        }
    }
}

impl ReaderConfig {
    pub fn upper_case(&self) -> bool {
        self.upper_case
    }

    pub fn offset_in_decimal(&self) -> bool {
        self.offset_in_decimal
    }

    pub fn group(&self) -> usize {
        self.group
    }

    pub fn seek(&self) -> usize {
        self.seek
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn bin(&self) -> bool {
        self.bin
    }
}

fn number(arg: &str, iter: &mut Iter<String>) -> Result<u64, Error> {
    match iter.next() {
        Some(value) => {
            match value.parse::<u64>() {
                Ok(v) => Ok(v),
                Err(_) => {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("{} should have number as next argument", arg),
                    ))
                }
            }
        },
        None => {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{} should have extra argument", arg),
            ))
        }
    }
}

impl TryFrom<Vec<String>> for ReaderConfig {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut res = ReaderConfig::default();

        let mut iter = value.iter();
        while let Some(item) = iter.next() {
            if item == "-d" {
                res.offset_in_decimal = true;
            }

            if item == "-u" {
                res.upper_case = true;
            }

            if item == "-c" {
                res.cols = number("-c", &mut iter)? as usize;
            }

            if item == "-s" {
                res.seek = number("-s", &mut iter)? as usize;
            }

            if item == "-g" {
                res.group = number("-g", &mut iter)? as usize;
            }

            if item == "-l" {
                res.len = number("-l", &mut iter)? as usize;
            }

            if item == "-b" {
                res.bin = true;
            }
        }

        if res.bin {
            res.group = 1;
            if res.cols == ReaderConfig::default().cols {
                res.cols = 6;
            }
        }

        Ok(res)
    }
}
