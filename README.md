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

By using the -v or --version flag the user is able to see the version of rssg

### Help

By using the -h or --help flag user is able to see help information

### Input

The program accepts inputs from the user using the -i or --input flag. Acceptable inputs are files and or folders

### Output
Output is stored in a current directory in folder named dist

### Formated HTML

Paragraphs are outputted in a formatted and clean way 

### Title Substitution

By having the first line as title with 2 empty lines following. A title tag and h1 is generated automatically in the html output

# Demo

![image](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ls1si3ltmz4kxis85cet.png)

is converted to

![image](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/yxot1qjnvcpnvp7q9fyz.png)
 

# USAGE

![image](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/jiekksl0twj6ehxpwl6r.png)
