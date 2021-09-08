use std::{env, fs, io::Result, process};

fn main() -> Result<()> {
    //Do not run the program until input is specified
    if env::args().count() == 1 {
        println!(
            "Please enter an existing directory or file location as an argument. Ex: rssg test.txt"
        );
        process::exit(1);
    }

    //Arguments passed to program
    let args: Vec<String> = env::args().collect();

    //Skip the first argument which is the executable and proceed
    args.iter().skip(1).for_each(|arg| {
        if let Ok(file) = fs::read_to_string(arg.to_owned()) {
            println!("{}", file);
        } else {
            println!("File named {} not found", arg);
        }
    });

    Ok(())
}
