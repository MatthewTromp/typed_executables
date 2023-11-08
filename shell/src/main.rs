use std::{io::{self, Write}, env, path::PathBuf, str::FromStr, fs::{File, self}, os::fd::AsRawFd, process::{Command, Stdio}, error::Error};

use text_io::read;

use nix::fcntl::{FdFlag, fcntl, FcntlArg};

fn get_command(path: &str) -> String {
    print!("[{}]$ ", path);
    io::stdout().flush().unwrap();
    read!("{}\n")
}

// Breaks the user input into a list of whitespace-separated strings
// TODO: handle quoting for file/directory names which contain whitespace
fn split(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn cd(path: &str) {
    // Try to open the new path
    if let Ok(new_path) = PathBuf::from_str(path) {
        if new_path.is_dir() {
            env::set_current_dir(new_path).unwrap();
        } else {
            println!("{} is not a directory", new_path.to_str().unwrap());
        }
    } else {
        println!("{} is not a valid path", path);
    }
}

fn touch(paths: &[&str]) -> Result<(), Box<dyn Error + 'static>> {
    for &path in paths {        
        File::create(path)?;
    }
    Ok(())
}

enum Status {
    Continue,
    Break,
}

fn run_typed_command(command: &str, args: &[&str]) -> Result<(), Box<dyn Error + 'static>> {
    // Filter out the filenames
    let mut files = vec![];
    let mut other_args = vec![];
 
    for arg in args {
        let mut chars = arg.chars();
        if chars.next().unwrap() == '\'' {
            let pathstr: String = chars.collect();
            // Open the file
            let file = fs::File::open(&pathstr).unwrap();
            files.push(file);
        } else {
            other_args.push(*arg);
        }
    }

    let fds: Vec<i32> = files.iter().map(
        |file| {
            let fd = file.as_raw_fd();
            let flags = FdFlag::from_bits(fcntl(fd, FcntlArg::F_GETFD)?).unwrap();
            // Set file descriptor to not close
            fcntl(fd, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC.complement().intersection(flags)))?;
            Ok::<i32, Box<dyn Error + 'static>>(fd)
        }).collect::<Result<Vec<_>, _>>()?;


    // Our args are going to be
    // 1. the number of file descriptor arguments
    // 2. the file descriptor arguments
    // 3. all the other arguments
    let mut fd_args = vec![format!("{:x}", fds.len())];
    fd_args.extend(fds.into_iter().map(|fd| format!("{:x}", fd)));

    let args: Vec<&str> = fd_args.iter().map(|s| s.as_str()).chain(other_args.into_iter()).collect();
  
    let mut child = Command::new(command)
        .args(args)
        .spawn()?;

    std::mem::drop(files); // Explicitly close files

    child.wait()?;
    
    Ok(())
}

fn run_normal_command(command: &str, args: &[&str]) -> Result<(), Box<dyn Error + 'static>> {
    let mut child = Command::new(command)
        .args(args)
        .spawn()?;

    child.wait()?;

    Ok(())
}

fn run_command(cmd: &[&str]) -> Result<(), Box<dyn Error + 'static>> {
    if cmd[0].chars().next().unwrap() == '\'' {
        // Untyped executable: conventional run
        run_normal_command(cmd[0].get(1..).unwrap(), &cmd[1..])
    } else {
        // Typed executable
        run_typed_command(cmd[0], &cmd[1..])
    }
}


fn take_command() -> Result<Status, Box<dyn Error + 'static>> {
    let raw_cmd = get_command(env::current_dir().unwrap().to_str().unwrap());
    let cmd = split(&raw_cmd);
    if cmd.len() > 0 {
        match cmd[0] {
            "exit" | "quit" => return Ok(Status::Break),
            "cd" => {
                if cmd.len() == 2 {
                    cd(cmd[1]);
                } else {
                    println!("cd takes 1 argument");
                }
            }
            "touch" => {
                if cmd.len() >= 2 {
                    touch(&cmd[1..])?;
                } else {
                    println!("touch takes at least 1 argument");
                }
            }
            _ => {
                // Run an actual command
                run_command(&cmd)?;
                // println!("Command: {}, args: {}", cmd[0], cmd.len()-1);
            }
        }
    }
    Ok(Status::Continue)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let file = File::create("test.txt").unwrap();
    println!("{:?}", file.as_raw_fd());
    loop {
        match take_command() {
            Ok(Status::Continue) => (),
            Ok(Status::Break) => break,
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
