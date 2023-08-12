mod editor;
mod terminal;

use std::error::Error;

pub use editor::Editor;
pub use terminal::Terminal;

type EdResult<T> = Result<T, Box<dyn Error>>;
const VERSION: &str = env!("CARGO_PKG_VERSION");
