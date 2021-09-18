use crate::parsing::input::{finalize_dist, read_text};
use std::{env, error::Error, process};

pub fn read_arguments() -> Result<(), Box<dyn Error>> {
    //Arguments passed to program
    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    //Matches passed flag and does appropiate action
    match &args[0] as &str {
        "-i" | "--input" => {
            //Do not run the program until input is specified check is here because -v is also arg
            //count of 2
            if env::args().count() == 2 {
                println!("Please enter input. Type rssg --help or -h for more information.");
                process::exit(0);
            }
            if read_text(&args[1..]).is_ok() {
                finalize_dist(args)
            } else {
                Err("Could not read files".into())
            }
        } //Pass args starting at 1 since 0 is the flag
        "-h" | "--help" => help_info(), //Prints help info
        "-v" | "--version" => {
            //Version num based on toml
            println!("rssg current version: {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => {
            //No valid flag passed
            println!("Please enter a valid flag. Type rssg --help or -h for more information.");
            process::exit(0);
        }
    }
}

//Prints Help information
fn help_info() -> Result<(), Box<dyn Error>> {
    println!("Rust Static Site Generator - RSSG");
    println!("\nUSAGE:");
    println!("  rssg [OPTIONS] [DIRECTORY/FILES]");
    println!("\nOPTIONS:");
    println!("  -h, --help         Prints help information");
    println!("  -v, --version      Prints rssg version");
    println!("  -i, --input        Specifies input to be used - directory or files");

    Ok(())
}
