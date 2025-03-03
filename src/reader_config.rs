use std::io::Error;

#[derive(Debug, Default)]
pub struct ReaderConfig {
    upper_case: bool,
    offset_in_decimal: bool,
}

impl ReaderConfig {
    pub fn upper_case(&self) -> bool {
        self.upper_case
    }

    pub fn offset_in_decimal(&self) -> bool {
        self.offset_in_decimal
    }
}

impl TryFrom<Vec<String>> for ReaderConfig {
    type Error = Error;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut res = ReaderConfig::default();

        for arg in value {
            if arg == "-d" {
                res.offset_in_decimal = true;
            }

            if arg == "-u" {
                res.upper_case = true;
            }
        }

        Ok(res)
    }
}
