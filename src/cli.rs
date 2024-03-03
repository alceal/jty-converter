use anyhow::{anyhow, Result};
use clap::{Args, Parser};

use crate::error::Error;

#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
pub struct Cli {
    /// The file to convert
    pub file: String,

    #[command(flatten)]
    pub format: Format,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Format {
    /// JSON output format
    #[arg(long)]
    pub json: bool,

    /// TOML output format
    #[arg(long)]
    pub toml: bool,

    #[arg(long, help = "YAML output format")]
    pub yaml: bool,
}

impl Cli {
    pub fn get_output_format(&self) -> Result<&str> {
        match (self.format.json, self.format.toml, self.format.yaml) {
            (true, false, false) => Ok("json"),
            (false, true, false) => Ok("toml"),
            (false, false, true) => Ok("yaml"),
            _ => Err(anyhow!(Error::UnsupportedOutputFormat)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_get_output_format() {
        let cli = Cli {
            file: "test.json".to_string(),
            format: Format {
                json: true,
                toml: false,
                yaml: false,
            },
        };
        assert_eq!(cli.get_output_format().unwrap(), "json");

        let cli = Cli {
            file: "test.toml".to_string(),
            format: Format {
                json: false,
                toml: true,
                yaml: false,
            },
        };
        assert_eq!(cli.get_output_format().unwrap(), "toml");

        let cli = Cli {
            file: "test.yaml".to_string(),
            format: Format {
                json: false,
                toml: false,
                yaml: true,
            },
        };
        assert_eq!(cli.get_output_format().unwrap(), "yaml");

        let cli = Cli {
            file: "test.json".to_string(),
            format: Format {
                json: false,
                toml: false,
                yaml: false,
            },
        };
        assert!(cli.get_output_format().is_err());
    }
}
