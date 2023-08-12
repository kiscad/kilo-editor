use crate::{editor, EdResult};

use crossterm::event::{Event, KeyEvent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, queue};
use std::io::Stdout;
use tui::backend::CrosstermBackend;

pub struct Terminal {
  out: Stdout,
  pub inner: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Default for Terminal {
  fn default() -> Self {
    Self::new()
  }
}

impl Terminal {
  #[must_use]
  pub fn new() -> Self {
    let out = std::io::stdout();
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = tui::Terminal::new(backend).unwrap();
    Self {
      out,
      inner: terminal,
    }
  }

  pub fn read_key(&self) -> EdResult<KeyEvent> {
    loop {
      if let Event::Key(event) = event::read()? {
        break Ok(event);
      }
    }
  }

  pub fn hide_cursor(&mut self) -> EdResult<()> {
    self.inner.hide_cursor()?;
    Ok(())
  }

  pub fn show_cursor(&mut self) -> EdResult<()> {
    self.inner.show_cursor()?;
    Ok(())
  }

  pub fn set_cursor(&mut self, pos: &editor::Position) -> EdResult<()> {
    self.inner.set_cursor(pos.x as u16, pos.y as u16)?;
    Ok(())
  }

  /// # Panics
  #[must_use]
  pub fn size(&self) -> (usize, usize) {
    let sz = self.inner.size().unwrap();
    (sz.width as usize, sz.height as usize)
  }

  /// # Errors
  pub fn draw_rows(&mut self) -> EdResult<()> {
    let height = self.inner.size()?.height;
    for row in 0..height - 1 {
      self.clear_current_line()?;
      if row == height / 3 {
        self.draw_welcome_message()?;
      } else {
        println!("~\r");
      }
    }
    print!("~\r");
    Ok(())
  }

  /// # Errors
  pub fn flush(&mut self) -> EdResult<()> {
    self.inner.flush()?;
    Ok(())
  }

  fn draw_welcome_message(&self) -> EdResult<()> {
    let welcome = format!("Hecto Editor -- version {}\r", crate::VERSION);
    let width = self.inner.size()?.width as usize;
    let padding = width.saturating_sub(welcome.len()) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    let mut message = format!("~{spaces}{welcome}");
    message.truncate(width);
    println!("{message}\r");
    Ok(())
  }

  fn clear_current_line(&mut self) -> EdResult<()> {
    queue!(self.out, Clear(ClearType::CurrentLine))?;
    Ok(())
  }

  /// # Panics
  pub fn clear(&mut self) {
    self.inner.clear().unwrap();
  }
}
