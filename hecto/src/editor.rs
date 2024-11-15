use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor { }

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
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
                            println!("{:?} ({c})\r", c as u8);
                        }
                    },
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?} \r"),
                }
                Err(e) => die(e)
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
