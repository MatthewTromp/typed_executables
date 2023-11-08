use std::{os::fd::RawFd, error::Error, env};

use nix::dir::Dir;

fn ls(fd: RawFd) -> Result<(), Box<dyn Error + 'static>> {
    match Dir::from_fd(fd) {
        Ok(dir) => {
            for thing in dir{
                println!("{}", thing?.file_name().to_str()?);
            }
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let num_args = usize::from_str_radix(&args[1], 16)?;
    for index in (0..num_args).map(|v| v+2) {
        let fd = RawFd::from_str_radix(&args[index], 16)?;
        ls(fd)?;
    }
    Ok(())
}
