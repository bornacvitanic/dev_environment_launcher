[![Test](https://github.com/bornacvitanic/dev_environment_launcher/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/dev_environment_launcher/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/bornacvitanic/dev_environment_launcher/status.svg)](https://deps.rs/repo/github/bornacvitanic/dev_environment_launcher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/dev_environment_launcher.svg)](https://crates.io/crates/dev_environment_launcher)
[![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/dev_environment_launcher/releases)

# Dev Environment Launcher

Dev Environment Launcher is a command-line tool to help manage and open development projects for  work environments like Unity and Rust.

## Features
- **Open Projects**: Easily open Unity and Rust projects from specified paths.
- **Recent Projects**: Keep track of recent projects and select from them interactively.
- **Configuration**: Customize IDE and editor paths for Unity and Rust.

# Roadmap

- **Auto-Completion**: Add shell auto-completion for commands and options.
- **Interactive Prompts Enhancements**: Improve the interactive menu to include project metadata and support batch operations.
- **Detailed Logs**: Add detailed logs for significant operations, including project opening, configuration changes, and errors.
- **Verbose Mode**: Add a verbose mode to display detailed logs in the console.
- **Support More Project/Environment Types**: Add support for various project types.
- **Modular Architecture**: Design the system to allow easy addition of new project types.
- **Project Templates**: Support multiple configurable environment templates for projects.


## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/bornacvitanic/dev_environment_launcher.git
    ```
2. Navigate to the project directory:
    ```sh
    cd dev_environment_launcher
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```
4. Run the executable:
    ```sh
    ./target/release/dev_environment_launcher
    ```

## Usage
```sh
Usage: devenv [SUBCOMMAND]

A tool to open development project workspaces

Subcommands:
    Path <path>        Specify a project path to open
    Open <index>       Specify a recent project index to open
    Remove <index>     Specify a recent project index to remove
    Clear              Clears all recent projects
    Options            List recent projects
    Recent             Interactive menu to select recent project to open
```

# Examples
- Open a project by path:
```sh
devenv path /path/to/your/project
```
- Open a recent project by index:
```sh
devenv open 2
```
- Remove a recent project by index:
```sh
devenv remove 1
```
- Clear all recent projects:
```sh
devenv clear
```
-List recent projects:
```sh
devenv options
```
-Interactive menu to select recent project:
```sh
devenv recent
```
# Configuration
Upon first run, the application creates a default configuration file at:

- Windows: %APPDATA%/dev_environment_launcher/config.toml
- macOS: ~/Library/Application Support/dev_environment_launcher/config.toml
- Linux: ~/.config/dev_environment_launcher/config.toml
# Configuration File

```toml
[rust]
ide_path = "path/to/rust/ide"

[unity]
editor_base_path = "path/to/unity/editor/base (before the version number folder)" 
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgements

- [structopt](https://docs.rs/structopt/0.3) - Library for setting up a basic command-line interface (CLI)
- [structopt-derive](https://docs.rs/structopt-derive/0.4) - Derive macros for `structopt`
- [dialoguer](https://docs.rs/dialoguer/0.11.0) - Library for handling user input
- [toml](https://docs.rs/toml/0.8.15) - Library for handling TOML config files
- [config](https://docs.rs/config/0.14.0) - Library for managing configuration files
- [serde](https://docs.rs/serde/1.0) - Library for serializing and deserializing Rust data structures
- [serde_derive](https://docs.rs/serde_derive/1.0) - Derive macros for `serde`

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/dev_environment_launcher/issues)