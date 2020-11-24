use std::fmt;
use std::path::PathBuf;

use crate::common::*;

#[derive(Debug)]
pub struct CompareWarning {
    pub path: PathBuf,
    pub missing_keys: Vec<String>,
}

impl CompareWarning {
    pub fn as_str(&self) -> String {
        format!(
            "{:?} is missing keys: {:?}",
            self.path,
            self.missing_keys
                .clone()
                .into_iter()
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: &str, message: String) -> Self {
        let check_name = String::from(check_name);
        Self {
            line,
            check_name,
            message,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line.number
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            format!("{}:{}", self.line.file, self.line.number).italic(),
            self.check_name.red().bold(),
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn warning_fmt_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(
            line,
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );

        assert_eq!(
            format!(
                "{} {}: {}",
                format!("{}:{}", ".env", "1").italic(),
                "DuplicatedKey".red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
