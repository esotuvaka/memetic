# Memetic

Memetic is a Rust CLI tool designed to read source code files, find the structs, analyze their memory layout, and suggest optimizations for a more optimal memory footprint. It provides a formatted output or a git diff to show the suggested changes.

## Features

-   **Recursive File Search**: Search for source code files starting from the directory the CLI is called in.
-   **Struct Analysis**: Identify and analyze the memory layout of structs in the source code.
-   **Optimization Suggestions**: Suggest a more optimal memory footprint for the structs, if any.
-   **Formatted Output**: Display the optimized memory layout in a readable format.
-   **Git Diff**: Show the suggested changes as a git diff.

## Installation

NOTE: Currently no full release yet, so will have to be built from source.

To install `memetic`, you need to have Rust and Cargo installed. Then, you can build the project using Cargo:

```sh
cargo build --release
```

## Usage

Run `memetic` with the following command:

```sh
memetic [OPTIONS]
```

### Options

-   -m mode: (E)xecute, (S)uggest, (D)iff using Git

-   -o override: Pass a json object that overrides defaults. e.g: {"type": "u32", "nat_align": 5, "size": 8}

-   -i includes: operate on files matching a comma separated list of regex patterns, instead of recursively

-   -e excludes: exclude files matching a comma separated list of regex patterns from recursive search

-   -d directory: operate from some starting directory, given some relative path to it. e.g: "./src"

## Examples

To analyze all files in the current directory and suggest optimizations:

```sh
memetic -a -m suggest
```

To auto implement changes over a nested directory, excluding 2 files:

```sh
memetic -m execute -d ./src/routers -e users.rs,messages.rs # note the lack of space between files
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
