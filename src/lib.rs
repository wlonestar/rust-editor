pub mod editor;
pub mod reader;
pub mod writer;

use crate::writer::Writer;
use crossterm::terminal;

const VERSION: &str = "0.1.0";
const TAB_SIZE: usize = 4;
const QUIT_TIMES: u8 = 2;

const BACKGROUND_COLOR: (u8, u8, u8) = (48, 56, 69);
const DEFAULT_COLOR: (u8, u8, u8) = (195, 211, 222);
const NUMBER_COLOR: (u8, u8, u8) = (209, 154, 102);
const CHAR_STRING_COLOR: (u8, u8, u8) = (146, 214, 158);
const COMMENT_COLOR: (u8, u8, u8) = (89, 98, 111);
const KEYWORDS_COLOR: (u8, u8, u8) = (199, 146, 234);

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
