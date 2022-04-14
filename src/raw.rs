use std::{os::unix::prelude::AsRawFd, error::Error};
use termios::{Termios, tcgetattr, tcsetattr};

pub fn get_orig_termios() -> Result<Termios, Box<dyn Error>>
{
    let fd = std::io::stdin().as_raw_fd();
    let mut orig_termios = Termios::from_fd(fd)?;
    tcgetattr(fd, &mut orig_termios)?;

    Ok(orig_termios)
}

pub fn enable_raw_mode() -> Result<(), Box<dyn Error>>
{
    let fd = std::io::stdin().as_raw_fd();
    let mut raw = Termios::from_fd(fd)?;

    tcgetattr(fd, &mut raw)?;
    raw.c_lflag &= !(termios::ECHO | termios::ICANON | termios::ISIG);
    tcsetattr(fd, termios::TCSAFLUSH, &raw)?;

    Ok(())
}

pub fn disable_raw_mode(orig_termios: Termios) -> Result<(), Box<dyn Error>>
{
    tcsetattr(std::io::stdin().as_raw_fd(), termios::TCSAFLUSH, &orig_termios)?;

    Ok(())
}