use core::time;
use std::io::{self, stdout, Write};
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

        println!("Welcome to Hecto (DEF version)\r");
        std::thread::sleep(time::Duration::from_secs(1)); // allows user to see message above
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }

            if self.should_quit {
                break;
            }

            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        if self.should_quit {
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1));
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
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
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
