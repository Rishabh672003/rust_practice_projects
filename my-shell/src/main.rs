extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::{env, path::Path, process::Command};

fn is_command(cmd: &str) -> bool {
    let path = env::var("PATH").unwrap();
    let paths = path.split(":").collect::<Vec<_>>();

    for i in paths {
        let os_path = Path::new(i);
        if !os_path.is_dir() {
            continue;
        }
        for dirs in os_path.read_dir().expect("failed").flatten() {
            if dirs.file_name().into_string().unwrap() == cmd {
                return true;
            }
        }
    }
    false
}

fn is_executable(cmd: &str) -> bool {
    let file = Path::new(cmd);
    let metadata = file.metadata();
    match metadata {
        Ok(val) => val.permissions().mode() & 0o111 != 0,
        Err(_) => false,
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("\x1b[0;32m$\x1b[0m ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let line = line.split_whitespace().collect::<Vec<&str>>();
                let Some(&name) = line.first() else { continue };
                if name == "exit" {
                    break;
                }
                if is_executable(name) || is_command(name) {
                    let args = line.iter().skip(1);
                    let output = Command::new(name).args(args).output()?;
                    io::stdout().write_all(&output.stdout)?;
                    if !output.stderr.is_empty() {
                        io::stderr().write_all(&output.stderr)?;
                    }
                    _ = io::stdout().flush();
                    continue;
                }
                println!("{name} is not executable");
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
