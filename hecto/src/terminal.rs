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
    (sz.width as usize, sz.height.saturating_sub(2) as usize)
  }

  /// # Errors
  pub fn flush(&mut self) -> EdResult<()> {
    self.inner.flush()?;
    Ok(())
  }

  pub fn clear_current_line(&mut self) -> EdResult<()> {
    queue!(self.out, Clear(ClearType::CurrentLine))?;
    Ok(())
  }

  /// # Panics
  pub fn clear(&mut self) {
    self.inner.clear().unwrap();
  }
}
