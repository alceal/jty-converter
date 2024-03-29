use anyhow::Result;

use crate::cli::Cli;
use crate::converter::Converter;

pub fn run(args: Cli) -> Result<()> {
    let file_path = &args.file;
    let output_format = args.get_output_format().unwrap();

    let converter = Converter::new(file_path, output_format);

    converter.convert()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::cli::Format;

    #[test]
    fn test_run_success() {
        let args = Cli {
            file: "tests/data/json/file.json".to_string(),
            format: Format {
                json: false,
                toml: true,
                yaml: false,
            },
        };

        let result = run(args);

        assert!(result.is_ok());

        if Path::new("tests/data/json/file.toml").exists() {
            std::fs::remove_file("tests/data/json/file.toml").unwrap();
        }
    }

    #[test]
    fn test_run_fails() {
        let args = Cli {
            file: "tests/data/json/file.json".to_string(),
            format: Format {
                json: true,
                toml: false,
                yaml: false,
            },
        };

        let result = run(args);

        assert!(result.is_err());
    }
}
