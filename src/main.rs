use std::{env, fs, io::Write, path::Path};

struct Args {
    appname: String,
    compiler: String, 
    standard: String,
}

impl Args {
    pub fn new(appname: String, compiler: String, standard: String) -> Self {
        return Self {appname, compiler, standard};
    }
}

fn main() {
    let mut args: Vec<String> = Vec::new();
    let mut appargs = Args::new("someapp".to_string(), "gcc".to_string(), "c99".to_string());
    let mut name_changed = false;
    let mut compiler_changed = false;
    let mut standard_changed = false;
    for arg in env::args() {
        args.push(arg);
    }

    dbg!(args.len());
    if args.len() > 1 && args[1] == "create" {
        if args.contains(&"-n".to_string()) {
            if let Some(index) = args.iter().position(|s| s == "-n") {
                if let Some(next_arg) = args.get(index + 1) {
                    appargs.appname = next_arg.clone();
                    name_changed = true;
                } else {
                    eprintln!("Error: Missing argument after -n");
                    return;
                }
            }
        }
        if let Some(index) = args.iter().position(|s| s == "-c") {
            if let Some(next_arg) = args.get(index + 1) {
                appargs.compiler = next_arg.clone();
                compiler_changed = true;
            } else {
                eprintln!("Error: Missing argument after -c");
                return;
            }
        }
        if let Some(index) = args.iter().position(|s| s == "-s") {
            if let Some(next_arg) = args.get(index + 1) {
                appargs.standard = next_arg.clone();
                standard_changed = true;
            } else {
                eprintln!("Error: Missing argument after -s");
                return;
            }
        }
            
        let file_path = Path::new("ezc.yml");
        dbg!(file_path);

        if file_path.exists() {
            println!("(0_o)  Warning: ezc.yml already exists! Overwriting...");
        }

        let mut file = match fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Unable to create the ezc.yml file: {}", e);
                return;
            }
        };

        match writeln!(file, "cc: {}", appargs.compiler) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to ezc.yml: {}", e);
                return;
            }
        }
        match writeln!(file, "src:") {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to ezc.yml: {}", e);
                return;
            }
        }

        match writeln!(file, "  - main.c") {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to ezc.yml: {}", e);
                return;
            }
        }

        match writeln!(file, "output: {}", appargs.appname) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to ezc.yml: {}", e);
                return;
            }
        }

        match writeln!(file, "std: {}", appargs.standard) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to ezc.yml: {}", e);
                return;
            }
        }

        println!("Done!");
        if name_changed {
            println!("Generated with app name \"{}\".", appargs.appname);
        }
        if compiler_changed {
            println!("Generated with compiler \"{}\".", appargs.compiler);
        }
        if standard_changed {
            println!("Generated with standard \"{}\".", appargs.standard);
        }
        return;
    } else if args.len() > 1 && args[1] == "help" {
        if args.len() > 2 {
            if args[2] == "create" {
                println!("ezc-gen create");
                println!("==================");
                println!("Creates an ezc.yml file. Usage:");
                println!("$ ezc-gen create");
                println!("Done! Check ezc.yml in your current directory for output.");
                println!("-----------------------------------------------------------------");
                println!("$ ezc-gen create -n someapp");
                println!("Done! Check ezc.yml in your current directory for output.");
                println!("Generated with app name \"someapp\".");
                println!("-----------------------------------------------------------------");
                println!("$ ezc-gen create -s c99");
                println!("Done! Check ezc.yml in your current directory for output.");
                println!("Generated with standard \"c99\".");
                println!("-----------------------------------------------------------------");
                println!("$ ezc-gen create -c gcc");
                println!("Done! Check ezc.yml in your current directory for output.");
                println!("Generated with compiler \"gcc\".");
                println!("-----------------------------------------------------------------");
                println!("$ ezc-gen create -n someapp -s c99 -c gcc");
                println!("Done! Check ezc.yml in your current directory for output.");
                println!("Generated with app name \"someapp\".\nGenerated with standard \"c99\".\nGenerated with compiler \"gcc\".");
                return;
            } else if args[2] == "help" {
                println!("ezc-gen help");
                println!("==================");
                println!("Shows the help message OR shows a more detailed help message when supplied with a command.");
                return;
            }
        }
        println!("ezc-gen version 0.1.0");
        println!("ezc-gen create | Creates a ezc.yml file.");
        println!("ezc-gen help | Shows this message");
        return;
    }
}
