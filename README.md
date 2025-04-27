# dir-mapper

>I wrote this tool in order to navigate projects and the code base, so to speak, "to look from afar"

A simple Rust command-line tool to display the directory structure in a tree-like format, similar to the tree command.
It supports customizable depth, hidden file visibility, and ignore patterns, with a clean and visually appealing output
using Unicode characters (└──, ├──, │).

## Features

- Displays the directory structure starting with the name of the current directory.
- Uses 4-space indentation for clear hierarchy visualization.
- Supports command-line arguments:
    - `-a`: Show hidden files (files starting with .).
    - -`D <depth>`: Limit the depth of the directory tree.
    - `-i <patterns>`: Ignore files or directories matching comma-separated patterns (e.g., `*.o`,`*.bin`).
- Sorts directories before files and alphabetically within each group.
- Unicode-based tree rendering for a polished look.

## Installation

#### Prerequisites

- Rust (stable, version 1.56 or later).
- Cargo (included with Rust).

#### Steps

Clone the repository:

```bash
git clone https://github.com/vremyavnikuda/dir-mapper.git
cd dir-mapper
```

Build the project:

```bash
cargo build --release
```

Run the executable:

```bash
./target/release/dir-mapper
```

Alternatively, install it globally using:

```bash
cargo install --path .
```

## Usage

Run the program from the command line, optionally specifying a directory path and flags.

Basic Command

```bash
dir-mapper [path] [-a] [-D depth] [-i patterns]
```

- `path`: Directory to display (defaults to current directory if not specified).
- `-a`: Include hidden files (e.g., `.gitignore`).
- `-D <depth>`: Maximum depth to display (e.g., `-D 2` for two levels).
- `-i <patterns>`: Comma-separated patterns to ignore (e.g., `*.o`,`*.lock`).

## Examples

Display the current directory's tree:

```bash
dir-mapper
```

Display a specific directory with hidden files:

```bash
dir-mapper /path/to/directory -a
```

Limit depth to 2 levels:

```bash
dir-mapper -D 2
```

Ignore specific file types:

```bash
dir-mapper -i "*.o,*.lock"
```

## Example Output

For a directory named `dir-mapper` with the following structure:

```bash
dir-mapper
├── src
│   ├── config.rs
│   ├── directory.rs
│   └── main.rs
├── target
│   ├── release
│   │   ├── build
│   │   ├── deps
│   │   │   ├── dir_mapper-288fb8db9b213114
│   │   │   └── dir_mapper-288fb8db9b213114.d
│   │   ├── examples
│   │   ├── incremental
│   │   ├── dir-mapper
│   │   └── dir-mapper.d
│   └── CACHEDIR.TAG
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss improvements or bugs.

1. Fork the repository.
2. Create a new branch (git checkout -b feature/your-feature).
3. Commit your changes (git commit -m 'Add your feature').
4. Push to the branch (git push origin feature/your-feature).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by the Unix tree command and built with Rust for performance and safety.