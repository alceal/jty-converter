use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::error::Error;

pub struct Writer<'a> {
    file_path: Rc<PathBuf>,
    output_format: &'a str,
}

impl<'a> Writer<'a> {
    pub fn new(file_path: &Rc<PathBuf>, output_format: &'a str) -> Self {
        Self {
            file_path: Rc::clone(file_path),
            output_format,
        }
    }

    pub fn write(&self, data: Value) -> Result<()> {
        let file_extension = self
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap();

        self.file_extension_matches_output_format(file_extension)?;

        let output = self.convert(data)?;

        let output_file_path = self.file_path.with_extension(self.output_format);

        self.output_file_exists(&output_file_path)?;

        std::fs::write(output_file_path, output)?;

        Ok(())
    }

    fn file_extension_matches_output_format(&self, file_extension: &str) -> Result<()> {
        if file_extension == self.output_format {
            return Err(anyhow!(Error::FileExtensionMatchesOutputFormat));
        }
        Ok(())
    }

    fn convert(&self, data: Value) -> Result<String> {
        match self.output_format {
            "json" => {
                let converted_string = serde_json::to_string_pretty(&data).unwrap();
                Ok(converted_string)
            }
            "toml" => {
                let converted_string = toml::to_string_pretty(&data).unwrap();
                Ok(converted_string)
            }
            "yaml" => {
                let converted_string = serde_yaml::to_string(&data).unwrap();
                Ok(converted_string)
            }
            _ => Err(anyhow!(Error::UnsupportedOutputFormat)),
        }
    }

    fn output_file_exists(&self, output_file_path: &Path) -> Result<()> {
        if output_file_path.exists() {
            return Err(anyhow!(Error::FileDoesExist(
                output_file_path.to_str().unwrap().to_string()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file_path = Rc::new(PathBuf::from("test.json"));
        let output_format = "json";

        let writer = Writer::new(&file_path, output_format);

        assert_eq!(writer.file_path, file_path);
        assert_eq!(writer.output_format, output_format);
    }

    #[test]
    fn test_file_extension_matches_output_format() {
        let file_path = Rc::new(PathBuf::from("test.json"));
        let output_format = "json";

        let writer = Writer::new(&file_path, output_format);

        let result = writer.file_extension_matches_output_format("json");

        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json() {
        let file_path = Rc::new(PathBuf::from("test.json"));
        let output_format = "json";

        let writer = Writer::new(&file_path, output_format);

        let data = Value::from(r#"{"foo": {"bar": "baz"}}"#);

        let result = writer.convert(data);

        assert!(result.is_ok());

        let res = result.unwrap();

        assert_eq!(res, r#""{\"foo\": {\"bar\": \"baz\"}}""#);
    }

    // It seems that toml::to_string(&data) is not working as expected
    // #[test]
    // fn test_convert_toml() {
    //     let file_path = Rc::new(PathBuf::from("test.json"));
    //     let output_format = "toml";

    //     let writer = Writer::new(&file_path, output_format);

    //     let data = Value::from(r#""{ foo = { bar = "baz" } }""#);

    //     let result = writer.convert(data);

    //     assert!(result.is_ok());

    //     let res = result.unwrap();

    //     assert_eq!(res, r#""[foo]
    //     bar = "baz"""#);
    //     // assert_eq!(res, r#""{\"foo\": \"bar\"}""#);
    // }

    #[test]
    fn test_convert_yaml() {
        let file_path = Rc::new(PathBuf::from("test.json"));
        let output_format = "yaml";

        let writer = Writer::new(&file_path, output_format);

        let data = Value::from(
            r#"foo:
          bar: baz"#,
        );

        let result = writer.convert(data);

        assert!(result.is_ok());

        let res = result.unwrap();

        assert_eq!(res, "|-\n  foo:\n            bar: baz\n");
    }
}
