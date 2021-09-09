use std::{env, fs, io::Result, process};

fn main() -> Result<()> {
    //Do not run the program until input is specified
    if env::args().count() == 1 {
        println!("Please enter input. Type rssg --help or -h for more information.");
        process::exit(1);
    }

    //Arguments passed to program
    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    //Matches passed flag and does appropiate action
    match &args[0] as &str {
        "-i" | "--input" => run(&args[1..]), //Pass args starting at 1 since 0 is the flag
        "-h" | "--help" => help_info(),
        "-v" | "--version" => println!("rssg current version: {}", env!("CARGO_PKG_VERSION")),
        _ => {
            println!("Please enter a valid flag. Type rssg --help or -h for more information.");
            process::exit(1)
        }
    }

    Ok(())
}

fn run(args: &[String]) {
    //Iterate through each input and process
    args.iter().for_each(|arg| {
        if let Ok(file) = fs::read_to_string(arg.to_owned()) {
            println!("{}", file);
        } else {
            println!("File named {} not found", arg);
        }
    });
}

//Prints Help information
fn help_info() {
    println!("Rust Static Site Generator - RSSG");
    println!("\nUSAGE:");
    println!("  rssg [OPTIONS] [DIRECTORY/FILES]");
    println!("\nOPTIONS:");
    println!("  -h, --help         Prints help information");
    println!("  -v, --version      Prints rssg version");
    println!("  -i, --input        Specifies input to be used - directory or files")
}
