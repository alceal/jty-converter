# jty-converter

![jty-converter](logo.webp)

`jty-converter` is a command-line tool written in Rust that converts between JSON, TOML, and YAML files.

It provides a simple and efficient way to convert data between these popular file formats. Whether you need to convert a JSON file to TOML, a TOML file to YAML, or any other combination, `jty-converter` has got you covered.

With `jty-converter`, you can easily manipulate and transform your data without the need for complex manual conversions. It handles the conversion process seamlessly, ensuring that your data remains intact and accurately transformed.

The tool is built using Rust, a fast and reliable programming language known for its performance and safety. Rust's strong type system and memory safety features make `jty-converter` a robust and dependable tool for file format conversions.

Whether you are a developer, data analyst, or system administrator, `jty-converter` simplifies the process of working with different file formats, saving you time and effort.

## Installation

### Pre-requisites

Before installing `jty-converter`, it's essential to have Rust programming language environment set up on your computer. To download and install Rust, please visit the [Rust official website](https://www.rust-lang.org/tools/install). This site provides detailed instructions for the installation process on various operating systems.

### Installing `jty-converter`

After ensuring Rust is properly installed, proceed with the installation of `jty-converter`. Open your terminal or command prompt and enter the following command:

```sh
cargo install jty-converter
```

This command compiles and installs the `jty-converter` package directly from crates.io, Rust's package registry.

## How to Use `jty-converter`

`jty-converter` is designed to facilitate the conversion between different configuration file formats: JSON, TOML, and YAML. To use it, the basic syntax is as follows:

```sh
jty-converter <format output> <FILE>
```

Where `<format output>` can be:

- `--json` to convert to JSON format.
- `--toml` to convert to TOML format.
- `--yaml` to convert to YAML format.

And `<FILE PATH>` is the path to the input file you wish to convert.

### Example

For instance, to convert a file named `config.json` into TOML format, execute:

```sh
jty-converter --toml config.json
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
