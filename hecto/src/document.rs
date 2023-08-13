use std::{fs, path::Path};

use crate::{row::Row, EdResult};

#[derive(Default)]
pub struct Document {
  pub fname: Option<String>,
  rows: Vec<Row>,
}

impl Document {
  pub fn open(fname: &dyn AsRef<Path>) -> EdResult<Self> {
    let content = fs::read_to_string(fname)?;
    let rows: Vec<_> = content.lines().map(|s| Row::from(s)).collect();
    let fname = Some(fname.as_ref().to_string_lossy().to_string());
    Ok(Self { rows, fname })
  }

  pub fn row(&self, index: usize) -> Option<&Row> {
    self.rows.get(index)
  }

  pub fn is_empty(&self) -> bool {
    self.rows.is_empty()
  }

  pub fn len(&self) -> usize {
    self.rows.len()
  }
}
