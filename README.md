## CLI Chess
[![crates.io](https://img.shields.io/crates/v/cli_chess)](https://crates.io/crates/cli_chess)
[![Pipeline Status](https://gitlab.com/DeveloperC/cli_chess/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/cli_chess/-/pipelines)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A program to play chess on the command line.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/cli_chess/-/releases](https://gitlab.com/DeveloperC/cli_chess/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```sh
git clone git@gitlab.com:DeveloperC/cli_chess.git
cd cli_chess/
cargo build --release
```

The compiled binary is present in `target/release/cli_chess`.


## Compiling via Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/cli_chess) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/cli_chess`.

```sh
cargo install cli_chess
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```sh
cargo install cli_chess --version 0.2.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```sh
cargo install cli_chess --version ^0
```

Will download the latest `0.*` release whether that is `0.2.0` or `0.4.9`.


## Unit Testing
The unit test suite has several parameterised tests, Cargo is used to set up and run all the unit tests.

```sh
cargo test
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://github.com/DeveloperC286/cli_chess/issues](https://github.com/DeveloperC286/cli_chess/issues).
