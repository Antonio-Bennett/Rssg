# Setting Up and Getting Started

## How to Install

- Download Rust from [Official Website](https://www.rust-lang.org/)
- Clone this repo
- Create sample text files or sample folder holding text files or both
- Build and run help using cargo r -- -h for usage or look below

## Example

`cargo run -- -i input.txt` <--- This converts a text file

`cargo run -- -i input.md` <--- This converts a markdown file

`cargo run -- -i input/` <--- This converts all valid files in a directory

`cargo run -- -h` <--- Shows more available options via help

# Formatting and Linting

With rust you get first class support for formatting and linting files. It makes it extremely simple.

`cargo fmt` <--- Run this to format the project

`cargo clippy` <--- Run this to lint the project

# IDE Integration

If you are using vim or neovim then formatting on save can be done via [null-ls](https://github.com/jose-elias-alvarez/null-ls.nvim)

Vscode users can modify the settings.json. However that is already handled

Both platforms can lint via the cli using `cargo clippy`
