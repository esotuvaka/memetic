# Structize

Structize is a Rust CLI tool designed to read source code files, find the structs, analyze their memory layout, and suggest optimizations for a more optimal memory footprint. It provides a formatted output or a git diff to show the suggested changes.

## Features

-   **Recursive File Search**: Search for source code files starting from the directory the CLI is called in.
-   **Struct Analysis**: Identify and analyze the memory layout of structs in the source code.
-   **Optimization Suggestions**: Suggest a more optimal memory footprint for the structs, if any.
-   **Formatted Output**: Display the optimized memory layout in a readable format.
-   **Git Diff**: Show the suggested changes as a git diff.

## Installation

!!! NOTE Currently no full release yet, so will have to be built from source.

To install Structize, you need to have Rust and Cargo installed. Then, you can build the project using Cargo:

```sh
cargo build --release
```

## Usage

Run Structize with the following command:

```sh
structize [OPTIONS]
```

### Options

-   -a, --all: Search all files recursively from the current directory, including files in .gitignore.
-   -o, --overrides: Pass a JSON object that overrides defaults. Example: {"type": "u32", "nat_align": 5, "size": 8}
-   -m, --mode: Specify the mode of operation. Options are execute, suggest, and diff.
-   -f, --files: Specify a comma-separated list of files to include in the search.
-   -e, --exclude: Specify a comma-separated list of files to exclude from the search.
-   -d, --directory: Operate starting from a specified starting directory.

## Examples

To analyze all files in the current directory and suggest optimizations:

```sh
structize -a -m suggest
```

To auto implement changes over a nested directory, excluding 2 files:

```sh
structize -m execute -d ./src/routers -e users.rs,messages.rs # note the lack of space between files
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
