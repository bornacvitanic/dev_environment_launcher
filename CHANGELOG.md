# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2024-07-28

### Documentation

- Add initial CHANGELOG.md file

- Update README.md to add crates.io badge

- Update README.md to add roadmap

- Update README.md to add dynamic badges


### Features

- Add support for opening json files to unity.rs and update config.rs to support specifying json editor path

- Add cliff.toml file for changelog generation

- Update Cargo.toml to add additional project fields


### Refactors

- Refactor main.rs to separate code out into methods


### Styling

- Style unity.rs, config.rs and main.rs using fmt

- Update files to format with fmt

- Update scripts to clean up unecessary borrows


### Testing

- Update unity.rs to add basic unit tests

- Update recent_projects.rs to add basic unit tests


## [0.1.0] - 2024-07-21

### Documentation

- Update README.md to fix Github Issues link

- Fix README.md license link

- Add LICENSE.md and README.md


### Features

- Update Cargo.toml to add more package metadata

- Add rust.yml Github workflow

- Update cli.rs to add clear command which clears all recent projects

- Update recent_projects.rs and cli.rs to add command line option to remove recent project from list

- Update Cli to add support for storing recent projects and quick access to them

- Add config library and config.rs to utilize toml configs for storing configuration data like paths to key programs

- Add rust.rs

- Add unity.rs

- Add project_type.rs

- Update main.rs to implement automatic project type detection based on the project folder structure

- Update main.rs to add capability of opening lazygits of packages and the packages folder

- Update utils.rs to add capability to launch Unity projects

- Update utils.rs to add lazygit starting functionality

- Add utils.rs to provide methods for opening files, directories and rust rover projects

- Add cli.rs to define Cli argument struct with project directory

- Add strucopt library


### Moves

- Update main.rs to move app name to const

- Update main.rs to move prompt_user_for_path to utils.rs

- Update utils.rs to move project type specific methods to appropriate files


### Renames

- Update cli.rs to rename Command enum options


### Updates

- Update recent_projects.rs to better format the menu options of recent projects

- Update dev_environment_launcher

- Update Cargo.toml and cli.rs to specify a custom name for the build executable

- Update main.rs to extract configuration directiory pickig logic to config.rs

- Update main.rs to prompt user for app path if it's missing in the config and to save it for future use

- Update main.rs to use Paths instead of raw strings for config path manipulation

- Update config to handle creation of default config file

- Update utils.rs to better handle errors when opening directories

- Update main.rs to remove unused imports

- Update cli.rs and main.rs to make the project directory argument optional and to use the current working directory if none is provided

- Update utils.rs to use start command to open files

- Update main.rs to open c# solution or rust project as specified via cli arguments

- Update main.rs to read command line arguments


<!-- generated by git-cliff -->
