use crossterm::terminal::{self};
use std::io;
use std::io::Read;

fn main() {
  terminal::enable_raw_mode().unwrap();

  for b in io::stdin().bytes() {
    let b = match b {
      Ok(b) => b,
      Err(e) => die(e),
    };
    let c = b as char;
    if c.is_control() {
      println!("{b:?}\r");
    } else {
      println!("{b:?} ({c})\r");
    }
    if b == to_ctrl_byte('q') {
      break;
    }
  }

  terminal::disable_raw_mode().unwrap();
}

fn to_ctrl_byte(c: char) -> u8 {
  let byte = c as u8;
  byte & 0b0001_1111
  // (c as u8) - b'a' + 1
}

fn die(e: io::Error) -> ! {
  panic!("{e}");
}
