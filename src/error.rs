use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported file extension")]
    UnsupportedFileExtension,

    #[error("Unsupported output format")]
    UnsupportedOutputFormat,

    #[error("Failed to write the output file")]
    FailedToWriteOutputFile(#[from] std::io::Error),

    #[error("File does exist: {0}")]
    FileDoesExist(String),

    #[error("File does not exist: {0}")]
    FileDoesNotExist(String),

    #[error("File extension matches the output format")]
    FileExtensionMatchesOutputFormat,

    #[error("Failed to read the JSON file: {0}")]
    FailedToReadJsonFile(#[from] serde_json::Error),

    #[error("Failed to read the TOML file: {0}")]
    FailedToReadTomlFile(#[from] toml::de::Error),

    #[error("Failed to read the YAML file: {0}")]
    FailedToReadYamlFile(#[from] serde_yaml::Error),
}
