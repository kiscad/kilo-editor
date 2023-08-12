mod document;
mod editor;
mod row;
mod terminal;

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

pub use editor::Editor;
pub use terminal::Terminal;

type EdResult<T> = Result<T, Box<dyn Error>>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
pub struct Config {
  fpath: Option<PathBuf>,
}
