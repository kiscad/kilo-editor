use clap::Parser;
use hecto::Config;
use hecto::Editor;

fn main() {
  let config = Config::parse();
  Editor::new(config).run();
}
