use std::{path::PathBuf, rc::Rc};

use anyhow::Result;

use crate::reader::Reader;
use crate::writer::Writer;

pub struct Converter<'a> {
    file_path: Rc<PathBuf>,
    output_format: &'a str,
}

impl<'a> Converter<'a> {
    pub fn new(file_path: &'a str, output_format: &'a str) -> Self {
        let file_path = PathBuf::from(file_path);

        Self {
            file_path: Rc::new(file_path),
            output_format,
        }
    }

    pub fn convert(&self) -> Result<()> {
        let reader = Reader::new(&self.file_path);

        let data = reader.read()?;

        let writer = Writer::new(&self.file_path, self.output_format);

        writer.write(data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_new() {
        let app = Converter::new("test.json", "json");

        assert_eq!(app.file_path.to_str().unwrap(), "test.json");
        assert_eq!(app.output_format, "json");
    }

    #[test]
    fn test_convert_success() {
        let file_path = "tests/data/json/file.json";
        let output_format = "toml";
        let app = Converter::new(file_path, output_format);

        let result = app.convert();

        assert!(result.is_ok());

        if Path::new("tests/data/json/file.toml").exists() {
            std::fs::remove_file("tests/data/json/file.toml").unwrap();
        }
    }

    #[test]
    fn test_convert_fails() {
        let file_path = "tests/data/json/file.json";
        let output_format = "json";
        let app = Converter::new(file_path, output_format);

        let result = app.convert();

        assert!(result.is_err());
    }
}
