use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/* fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
} */

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // command raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    println!("Enter characters:\r");
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?} \r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                },
                Key::Ctrl('q') => break,
                _ => println!("{:?} \r", key),
            }
            Err(e) => die(e)
        }
    }
}
