#[derive(Debug)]
pub struct Row {
  string: String,
}

impl Default for Row {
  fn default() -> Self {
    Row {
      string: String::new(),
    }
  }
}

impl From<&str> for Row {
  fn from(value: &str) -> Self {
    Self {
      string: value.replace("\t", "     "),
    }
  }
}

impl Row {
  pub fn render(&self, start: usize, end: usize) -> String {
    let end = self.string.len().min(end);
    let start = start.min(end);
    self.string.get(start..end).unwrap_or_default().to_string()
  }

  pub fn len(&self) -> usize {
    self.string.chars().count()
  }
}
