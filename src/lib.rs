pub mod cmd;
pub mod lsp;

use serde::{Serialize, Deserialize};
use std::{ env::{self, args}, process, path::{self, PathBuf}, fs, io };

pub use cmd::{Args, Argument, Opt, Flag, Subcommand, Cmd, };

pub struct Server {}

pub async fn init() {
    
}
