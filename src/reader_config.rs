use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct ReaderConfig {
    upper_case: bool,
    offset_in_decimal: bool,
    group: usize,
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            upper_case: false,
            offset_in_decimal: false,
            group: 4,
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

            if item == "-g" {
                match iter.next() {
                    Some(value) => {
                        res.group = match value.parse::<usize>() {
                            Ok(v) => v,
                            Err(_) => {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "-g should have number as next argument",
                                ))
                            }
                        };
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "-g should have extra argument",
                        ))
                    }
                }
            }
        }

        Ok(res)
    }
}
