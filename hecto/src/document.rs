use std::{fs, path::Path};

use crate::{row::Row, EdResult};

#[derive(Default)]
pub struct Document {
  rows: Vec<Row>,
}

impl Document {
  pub fn open(fname: &dyn AsRef<Path>) -> EdResult<Self> {
    let content = fs::read_to_string(fname)?;
    let rows: Vec<_> = content.lines().map(|s| Row::from(s)).collect();
    Ok(Self { rows })
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
