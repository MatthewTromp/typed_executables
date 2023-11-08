use std::{os::fd::{RawFd, FromRawFd}, error::Error, env, fs::File, io::{Read, Write, self}};

fn cat(fd: RawFd) -> Result<(), Box<dyn Error + 'static>> {
    let mut file = unsafe {File::from_raw_fd(fd)};
    let mut buf: [u8; 4096] = [0; 4096];
    while let Ok(num) = file.read(&mut buf) {
        if num == 0 {
            break;
        }
        io::stdout().write(&buf[0..num])?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let num_args = usize::from_str_radix(&args[1], 16)?;
    for index in (0..num_args).map(|v| v+2) {
        let fd = RawFd::from_str_radix(&args[index], 16)?;
        cat(fd)?;
    }
    io::stdout().flush()?;
    Ok(())
}
