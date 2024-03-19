use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{anyhow, Result};
use serde_json::Value; // We need to import the Value type from the toml crate or toml or serde_yaml

use crate::error::Error;

pub struct Reader {
    pub file_path: Rc<PathBuf>,
}

impl Reader {
    pub fn new(file_path: &Rc<PathBuf>) -> Self {
        Self {
            file_path: Rc::clone(file_path),
        }
    }

    pub fn read(&self) -> Result<Value> {
        self.check_file_exists(&self.file_path)?;

        let input_string = std::fs::read_to_string(self.file_path.as_ref()).unwrap();

        let file_extension = self
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap();

        let data = match file_extension {
            "json" => serde_json::from_str::<Value>(&input_string)
                .map_err(|e| anyhow!(Error::FailedToReadJsonFile(e)))?,
            "toml" => toml::from_str::<Value>(&input_string)
                .map_err(|e| anyhow!(Error::FailedToReadTomlFile(e)))?,
            "yaml" => serde_yaml::from_str::<Value>(&input_string)
                .map_err(|e| anyhow!(Error::FailedToReadYamlFile(e)))?,
            _ => return Err(anyhow!(Error::UnsupportedFileExtension)),
        };

        Ok(data)
    }

    fn check_file_exists(&self, file_path: &Path) -> Result<()> {
        if !file_path.exists() {
            return Err(anyhow!(Error::FileDoesNotExist(
                file_path.to_str().unwrap().to_string()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_new() {
        let file_path = Rc::new(PathBuf::from("test.json"));
        let reader = Reader::new(&file_path);
        assert_eq!(reader.file_path, file_path);
    }

    #[test]
    fn test_reader_read_json() {
        let file_path = Path::new("tests/data/json/file.json");

        let reader = Reader::new(&Rc::new(file_path.to_path_buf()));

        let result = reader.read();

        assert!(result.is_ok());

        let data = result.unwrap().to_string();

        let expected = r#"{"foo":{"bar":"baz"}}"#.to_string();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_reader_read_toml() {
        let file_path = Path::new("tests/data/toml/file.toml");

        let reader = Reader::new(&Rc::new(file_path.to_path_buf()));

        let result = reader.read();

        assert!(result.is_ok());

        let data = result.unwrap().to_string();

        let expected = r#"{"foo":{"bar":"baz"}}"#.to_string();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_reader_read_yaml() {
        let file_path = Path::new("tests/data/yaml/file.yaml");

        let reader = Reader::new(&Rc::new(file_path.to_path_buf()));

        let result = reader.read();

        assert!(result.is_ok());

        let data = result.unwrap().to_string();

        let expected = r#"{"foo":{"bar":"baz"}}"#.to_string();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_file_doesnt_exists() {
        let file_path = Path::new("test.txt");

        let reader = Reader::new(&Rc::new(file_path.to_path_buf()));

        let result = reader.read();

        assert!(result.is_err());

        let err = result.err().unwrap().to_string();

        let expected = "File does not exist: test.txt".to_string();

        assert_eq!(err, expected);
    }

    #[test]
    fn test_unsupported_file_extension() {
        let file_path = Path::new("tests/data/unsupported/file.txt");

        let reader = Reader::new(&Rc::new(file_path.to_path_buf()));

        let result = reader.read();

        assert!(result.is_err());

        let err = result.err().unwrap().to_string();

        let expected = "Unsupported file extension".to_string();

        assert_eq!(err, expected);
    }
}
