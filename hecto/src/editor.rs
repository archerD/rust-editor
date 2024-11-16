use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {should_quit: false}
    }

    pub fn run(&mut self) {
        // command raw mode
        let _stdout = stdout().into_raw_mode().unwrap();

        println!("Enter characters:\r");
        while !self.should_quit {
            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Char(c) => {
                println!("{c}\r");
            },
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
