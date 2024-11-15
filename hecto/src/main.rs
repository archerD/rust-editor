use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // command raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    println!("Enter characters");
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }

                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => die(e)
        }
    }
}
