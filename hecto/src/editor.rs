use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;

use crate::{EdResult, Terminal};

/// Position in editor may be different with the position in terminal.
#[derive(Debug, Default)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

pub struct Editor {
  pub should_quit: bool,
  terminal: Terminal,
  position: Position,
}

impl Default for Editor {
  fn default() -> Self {
    Self::new()
  }
}

impl Editor {
  #[must_use]
  pub fn new() -> Self {
    let terminal = Terminal::new();
    let position = Position::default();
    Self {
      should_quit: false,
      terminal,
      position,
    }
  }

  /// run
  /// # Panics
  pub fn run(&mut self) {
    enable_raw_mode().unwrap();

    loop {
      if let Err(error) = self.refresh_screen() {
        self.die(error.as_ref());
      }
      if self.should_quit {
        break;
      }
      let key_res = self.terminal.read_key();
      match key_res {
        Err(e) => self.die(e.as_ref()),
        Ok(event) => self.process_keypress(event),
      }
    }
    disable_raw_mode().unwrap();
  }

  fn refresh_screen(&mut self) -> EdResult<()> {
    self.terminal.hide_cursor()?;
    self.terminal.set_cursor(&self.position)?;
    if self.should_quit {
      self.terminal.set_cursor(&Position::default())?;
      self.terminal.clear();
      println!("Goodbye.\r");
    } else {
      self.terminal.set_cursor(&Position::default())?;
      self.terminal.draw_rows()?;
      self.terminal.set_cursor(&self.position)?;
    }
    self.terminal.show_cursor()?;
    self.terminal.flush()
  }

  fn process_keypress(&mut self, event: KeyEvent) {
    match event {
      KeyEvent {
        modifiers: KeyModifiers::CONTROL,
        code,
        ..
      } => match code {
        KeyCode::Char('q') => self.should_quit = true,
        KeyCode::Char(c) => println!("CTRL + {c}\r"),
        _ => {}
      },
      KeyEvent {
        modifiers: KeyModifiers::NONE,
        code: KeyCode::Char(c),
        ..
      } => println!("{c}\r"),
      KeyEvent {
        modifiers: KeyModifiers::NONE,
        code,
        ..
      } => match code {
        KeyCode::Left
        | KeyCode::Right
        | KeyCode::Up
        | KeyCode::Down
        | KeyCode::PageUp
        | KeyCode::PageDown
        | KeyCode::End
        | KeyCode::Home => self.move_cursor(code),
        _ => {}
      },
      _ => {}
    }
  }

  fn move_cursor(&mut self, code: KeyCode) {
    let Position { mut x, mut y } = self.position;
    let (width, height) = self.terminal.size();
    match code {
      KeyCode::Left => x = x.saturating_sub(1),
      KeyCode::Right => {
        if x < width {
          x = x.saturating_add(1)
        }
      }
      KeyCode::Up => y = y.saturating_sub(1),
      KeyCode::Down => {
        if y < height {
          y = y.saturating_add(1)
        }
      }
      KeyCode::PageUp => y = 0,
      KeyCode::PageDown => y = height,
      KeyCode::Home => x = 0,
      KeyCode::End => x = width,
      _ => (),
    }
    self.position = Position { x, y };
  }

  fn die(&mut self, error: &dyn Error) -> ! {
    self.terminal.clear();
    panic!("{error}");
  }
}
