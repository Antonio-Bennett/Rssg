### What is this project?

This is a simple [Static Site Generator](https://www.cloudflare.com/en-ca/learning/performance/static-site-generator/#:~:text=A%20static%20site%20generator%20is,and%20a%20set%20of%20templates.&text=Static%20site%20generators%20are%20an,generating%20webpages%2C%20and%20implementing%20templates.) that converts txt files to html

## Sample Site

[Rssg Sample](https://antonio-bennett.github.io/)

# How to Install

- Download Rust from [Official Website](https://www.rust-lang.org/)
- Clone this repo
- Create sample text files or sample folder holding text files or both
- Build and run help using cargo r -- -h for usage or look below

# Features

### Version

By using the -v or --version flag `cargo r -- -v` the user is able to see the version of rssg

### Help

By using the -h or --help flag `cargo r -- -h`  the user is able to see help information

### Input

The program accepts inputs from the user using the -i or --input flag. Acceptable inputs are files and or folders
```rust
cargo r -- -i example.txt
cargo r -- -i exampleFolder/
```

### Output

Output is stored in a current directory in folder named dist

### Formated HTML

Paragraphs are outputted in a formatted and clean way

### Title Substitution

By having the first line as title with 2 empty lines following. A title tag and h1 is generated automatically in the html output

# Demo

```
Test File


A small sample size

Nothing major I just wnat to Test
how good this thing is working

Test test test test
```

is converted to

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Test File</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <h1>Test File</h1>

    <p>A small sample size</p>

    <p>Nothing major I just wnat to Test how good this thing is working</p>

    <p>Test test test test</p>
  </body>
</html>
```

# USAGE

```
Rust Static Site Generator - RSSG

USAGE:
  rssg [OPTIONS] [DIRECTORY/FILES]

  OPTIONS:
  -h, --help         Prints help information
  -v, --version      Prints rssg version
  -i, --input        Specifies input to be used - directory or files
```
