pub mod editor;
pub mod reader;
pub mod writer;

use crate::writer::Writer;
use crossterm::terminal;

const VERSION: &str = "0.1.0";
const TAB_SIZE: usize = 4;

pub struct CleanUp;

// drop() is called in cases such as
// when the instance of the struct goes out of scope normally or
// when there's a panic while the instance is still in scope.
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Writer::clear_screen().expect("Error");
    }
}
