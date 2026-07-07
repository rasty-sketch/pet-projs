use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("mysh>");

        io::stdout().flush().expect("failed to flush");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("err");

        let cleaned_input = input.trim();
        let mut input_iter = cleaned_input.split_whitespace();

        let first_word = input_iter.next().unwrap_or("");

        let args = input_iter.by_ref().take_while(|&x| x != ">").collect();
        let file: Vec<_> = input_iter.collect();

        match first_word {
            "cd" => {
                cd_handler(args);
            }
            "exit" => {
                println!("Hope to see you soon!!");
                break;
            }
            "" => {
                continue;
            }
            x => handler(x, args, file),
        }
    }
}

pub fn handler(c: &str, a: Vec<&str>, file: Vec<&str>) {
    let mut dest_file: Option<&str> = None;

    if file.len() == 1 {
        dest_file = Some(file[0])
    }

    let f = if let Some(file) = dest_file { file } else { "" };

    match f {
        "" => {}
        x => {
            let file = match File::create(x) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("failed to create file {x} error: {e}");
                    return;
                }
            };

            let mut child = match Command::new(c).args(a.clone()).stdout(file).spawn() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("failed to execute {c}, due to {e}");
                    return;
                }
            };

            match child.wait() {
                Ok(_x) => {}
                Err(e) => eprintln!("err is {e}"),
            }

            return;
        }
    }

    let output = Command::new(c).args(a).status();

    match output {
        Ok(status) => {
            if !status.success() {
                eprintln!("exited with status: {status}");
            }
        }
        Err(e) => {
            eprintln!("failed to execute command {}: {}", c, e);
        }
    }
}

pub fn cd_handler(args: Vec<&str>) {
    let target_dir = args.first().unwrap_or(&"/home");
    let root = Path::new(&target_dir);

    if let Err(e) = env::set_current_dir(root) {
        eprintln!("failed to change dir: Error {}", e)
    } else {
        println!("moved to {:?}", root)
    }
}
