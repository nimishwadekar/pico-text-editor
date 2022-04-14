mod raw;

use std::{io::{stdin, Read}, error::Error};

fn editor() -> Result<(), Box<dyn Error>>
{
    raw::enable_raw_mode()?;

    let mut buf: [u8; 1] = [0; 1];
    while let Ok(read) = stdin().read(&mut buf)
    {
        if read < 1 || buf[0] as char == 'q'
        {
            break;
        }

        if buf[0] < 32 || buf[0] == 127
        {
            println!("{}", buf[0]);
        }
        else
        {
            println!("{} ('{}')", buf[0], buf[0] as char);
        }
    }

    Ok(())
}

fn main()
{
    let orig_termios = raw::get_orig_termios().unwrap();

    if let Err(err) = editor()
    {
        eprintln!("{}", err);
    }

    raw::disable_raw_mode(orig_termios).unwrap();
}
