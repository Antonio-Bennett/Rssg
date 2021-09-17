use std::{
    env,
    fs::{self, OpenOptions},
    io::{Result, Write},
    path::Path,
    process,
};

fn main() -> Result<()> {
    //Do not run the program until input is specified
    if env::args().count() == 2 {
        println!("Please enter input. Type rssg --help or -h for more information.");
        process::exit(0);
    }

    //Arguments passed to program
    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    //Matches passed flag and does appropiate action
    match &args[0] as &str {
        "-i" | "--input" if run(&args[1..]).is_ok() => {
            if Path::new("./dist/").is_dir() {
                fs::remove_dir_all("./dist/")?;
            }

            fs::create_dir("./dist")?;
            args.into_iter().skip(1).for_each(|file| {
                let mut html = String::new();
                if Path::new(&file).is_dir() {
                    for entry in fs::read_dir(&file).unwrap() {
                        html = entry
                            .unwrap()
                            .path()
                            .to_string_lossy()
                            .to_string()
                            .strip_prefix(&file)
                            .unwrap()
                            .strip_suffix(".txt")
                            .unwrap()
                            .to_owned()
                            + ".html";

                        let new_location = "./dist/".to_owned() + &html;
                        fs::copy(&html, &new_location).unwrap();
                        fs::remove_file(html).unwrap();
                    }
                } else {
                    html = file.strip_suffix(".txt").unwrap().to_owned() + ".html";
                    let new_location = "./dist/".to_owned() + &html;
                    fs::copy(&html, new_location).unwrap();
                    fs::remove_file(html).unwrap();
                }
            })
        } //Pass args starting at 1 since 0 is the flag
        "-h" | "--help" => help_info(),
        "-v" | "--version" => println!("rssg current version: {}", env!("CARGO_PKG_VERSION")),
        _ => {
            println!("Please enter a valid flag. Type rssg --help or -h for more information.");
            process::exit(0)
        }
    }

    Ok(())
}

fn run(args: &[String]) -> Result<()> {
    //Iterate through each input and process
    args.iter().for_each(|arg| {
        if let Ok(mut file) = fs::read_to_string(arg.to_owned()) {
            process(&mut file, arg);
        } else if Path::new(arg).is_dir() {
            let path = Path::new(arg);
            visit_dirs(path, &process).expect("Couldn't convert dir");
        }
    });

    Ok(())
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&mut String, &str)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else if let Ok(mut file) = fs::read_to_string(&path) {
                println!("In visit dirs");
                let pathname = path.to_string_lossy();
                let filename = pathname.strip_prefix(dir.to_str().unwrap()).unwrap();
                cb(&mut file, filename);
            }
        }
    }
    Ok(())
}

fn process(file: &mut String, filename: &str) {
    let name = filename.strip_suffix(".txt").unwrap();
    let name = &(name.to_owned() + ".html");

    let mut html = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .unwrap();

    let vec_lines: Vec<&str> = file.lines().into_iter().collect();
    let mut line = String::new();

    let default_content = "<!doctype html>
<html lang=\"en\">
<head>
\t<meta charset=\"utf-8\">";

    html.write_all(default_content.as_bytes()).unwrap();

    if vec_lines[1].is_empty() && vec_lines[2].is_empty() && !vec_lines[0].is_empty() {
        let default_content = &("\n\t<title>".to_owned() + vec_lines[0] + "</title>");
        html.write_all(default_content.as_bytes()).unwrap();

        let default_content = "
\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
</head>
<body>\n";

        html.write_all(default_content.as_bytes()).unwrap();
        html.write_all(("\t<h1>".to_owned() + vec_lines[0] + "</h1>\n\n").as_bytes())
            .unwrap();

        vec_lines.into_iter().skip(3).for_each(|curr_line| {
            if !curr_line.is_empty() {
                line = "\t<p>".to_owned() + curr_line + "</p>\n";
                html.write_all(line.as_bytes())
                    .expect("Could not write to file");
            } else {
                html.write_all("\n".as_bytes())
                    .expect("Could not write to file");
            }
        });
    } else {
        html.write_all(default_content.as_bytes()).unwrap();
        let default_content = &("\n\t<title>".to_owned() + name + "</title>");
        html.write_all(default_content.as_bytes()).unwrap();

        let default_content = "
\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
</head>
<body>\n";

        html.write_all(default_content.as_bytes()).unwrap();

        vec_lines.into_iter().for_each(|curr_line| {
            if !curr_line.is_empty() {
                line = "\t<p>".to_owned() + curr_line + "</p>\n";
                html.write_all(line.as_bytes())
                    .expect("Could not write to file");
            } else {
                html.write_all("\n".as_bytes())
                    .expect("Could not write to file");
            }
        });
    }

    let default_content = "
</body>
</html>";

    html.write_all(default_content.as_bytes()).unwrap();
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
