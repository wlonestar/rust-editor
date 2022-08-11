use crossterm::terminal;

pub struct CleanUp;

// drop() is called in cases such as
// when the instance of the struct goes out of scope normally or
// when there's a panic while the instance is still in scope.
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}
