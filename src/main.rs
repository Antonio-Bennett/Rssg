use std::{
    env,
    fs::{self, OpenOptions},
    io::{Result, Write},
    process,
};

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
        if let Ok(mut file) = fs::read_to_string(arg.to_owned()) {
            process(&mut file, arg);
        } else {
            println!("File named {} not found", arg);
        }
    });
}

fn process(file: &mut String, filename: &str) {
    let name = filename.strip_suffix(".txt").unwrap();
    let name = &(name.to_owned() + ".html");

    let mut html = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(name)
        .unwrap();

    // let mut num_of_emp: u8 = 0;
    // let mut line_content = String::new();

    let vec_lines: Vec<&str> = file.lines().into_iter().collect();
    let mut line = String::new();

    if vec_lines[1].is_empty() && vec_lines[2].is_empty() && !vec_lines[0].is_empty() {
        line = "<title>".to_owned() + vec_lines[0] + "</title>";

        html.write_all(line.as_bytes())
            .expect("Could not write to file");

        vec_lines.into_iter().skip(3).for_each(|curr_line| {
            if !curr_line.is_empty() {
                line = "<p>".to_owned() + curr_line + "</p>";
                html.write_all(line.as_bytes())
                    .expect("Could not write to file");
            } else {
                html.write_all("\n\n".as_bytes())
                    .expect("Could not write to file");
            }
        })
    } else {
        vec_lines.into_iter().for_each(|curr_line| {
            if !curr_line.is_empty() {
                line = "<p>".to_owned() + curr_line + "</p>";
                html.write_all(line.as_bytes())
                    .expect("Could not write to file");
            } else {
                html.write_all("\n\n".as_bytes())
                    .expect("Could not write to file");
            }
        })
    }

    //     file.lines().into_iter().for_each(|line| {
    //         //Store line with content count num of empty lines between content and modify prev content as needed
    //         if !line.is_empty() {
    //             if num_of_emp == 1 {
    //                 num_of_emp = 0;
    //                 line_content = "<p>".to_owned() + &line_content + "</p>";
    //                 //write content to new file in place instead of storing in vec to join then write
    //                 html.write_all(line_content.as_bytes())
    //                     .expect("Could not write to file");

    //                 line_content = String::new();
    //             } else if num_of_emp == 2 {
    //                 num_of_emp = 0;
    //                 line_content = "<title>".to_owned() + &line_content + "</title>";
    //                 //write content to new file in place instead of storing in vec to join then write
    //                 html.write_all(line_content.as_bytes())
    //                     .expect("Could not write to file");

    //                 line_content = String::new();
    //             } else {
    //                 line_content = line.to_owned();
    //             }
    //         } else {
    //             num_of_emp += 1;
    //             html.write_all("\n".as_bytes())
    //                 .expect("Could not write empty line to file");
    //         }
    //     })
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
