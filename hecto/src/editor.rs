use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;

use crate::document::Document;
use crate::row::Row;
use crate::{Config, EdResult, Terminal};

/// Position in editor may be different with the position in terminal.
#[derive(Debug, Default)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[allow(dead_code)]
pub struct Editor {
  config: Config,
  document: Document,
  pub should_quit: bool,
  terminal: Terminal,
  cur_pos: Position,
  offset: Position,
}

impl Editor {
  #[must_use]
  pub fn new(config: Config) -> Self {
    let terminal = Terminal::new();
    let cur_pos = Position::default();
    let document = if let Some(path) = &config.fpath {
      Document::open(path).unwrap()
    } else {
      Document::default()
    };
    let offset = Position::default();

    Self {
      config,
      should_quit: false,
      terminal,
      cur_pos,
      document,
      offset,
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
    let term_pos = Position {
      x: self.cur_pos.x - self.offset.x,
      y: self.cur_pos.y - self.offset.y,
    };
    assert_eq!(self.terminal.inner.get_cursor().unwrap(), (0, 0));
    // self.terminal.set_cursor(&term_pos)?;
    if self.should_quit {
      self.terminal.set_cursor(&Position::default())?;
      self.terminal.clear();
      println!("Goodbye.\r");
    } else {
      self.terminal.set_cursor(&Position::default())?;
      self.draw_rows()?;
      self.terminal.set_cursor(&term_pos)?;
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
        | KeyCode::Home => {
          self.move_cursor(code);
          self.update_offset();
        }
        _ => {}
      },
      _ => {}
    }
  }

  fn move_cursor(&mut self, code: KeyCode) {
    let Position { mut x, mut y } = self.cur_pos;
    let row_len = self.document.row(y).map_or(0, |r| r.len());
    let doc_len = self.document.len();
    let (_, height) = self.terminal.size();
    match code {
      KeyCode::Left => {
        if x > 0 {
          x -= 1;
        } else if y > 0 {
          y -= 1;
          x = self.document.row(y).map_or(0, |r| r.len());
        }
      }
      KeyCode::Right => {
        if x < row_len {
          x = x.saturating_add(1)
        } else if y < doc_len {
          y += 1;
          x = 0;
        }
      }
      KeyCode::Up => y = y.saturating_sub(1),
      KeyCode::Down => {
        if y < doc_len {
          y = y.saturating_add(1)
        }
      }
      KeyCode::PageUp => y = y.saturating_sub(height),
      KeyCode::PageDown => y = (y + height).min(doc_len),
      KeyCode::Home => x = 0,
      KeyCode::End => x = row_len,
      _ => (),
    }

    let width = self.document.row(y).map_or(0, |r| r.len());
    x = x.min(width);

    self.cur_pos = Position { x, y };
  }

  fn update_offset(&mut self) {
    let Position { x, y } = self.cur_pos;
    let (wd, ht) = self.terminal.size();
    if x < self.offset.x {
      self.offset.x = x;
    } else if x >= self.offset.x + wd {
      self.offset.x = x - wd + 1;
    }
    if y < self.offset.y {
      self.offset.y = y;
    } else if y >= self.offset.y + ht {
      self.offset.y = y - ht + 1;
    }
  }

  fn die(&mut self, error: &dyn Error) -> ! {
    self.terminal.clear();
    panic!("{error}");
  }

  pub fn draw_row(&self, row: &Row) {
    let start = self.offset.x;
    let width = self.terminal.size().0;
    let end = self.offset.x + width;
    let row = row.render(start, end);
    println!("{row}\r");
  }

  fn draw_rows(&mut self) -> EdResult<()> {
    let height = self.terminal.size().1;
    for term_row in 0..height {
      self.terminal.clear_current_line()?;
      if let Some(row) = self.document.row(term_row + self.offset.y) {
        self.draw_row(row);
      } else if self.document.is_empty() && term_row == height / 3 {
        self.draw_welcome_message()?;
      } else {
        println!("~\r");
      }
    }
    Ok(())
  }

  fn draw_welcome_message(&self) -> EdResult<()> {
    let welcome = format!("Hecto Editor -- version {}\r", crate::VERSION);
    let width = self.terminal.size().0;
    let padding = width.saturating_sub(welcome.len()) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    let mut message = format!("~{spaces}{welcome}");
    message.truncate(width);
    println!("{message}\r");
    Ok(())
  }
}
