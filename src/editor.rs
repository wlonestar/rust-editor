use crate::reader::Reader;
use crate::writer::Writer;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Editor struct
pub struct Editor {
    reader: Reader,
    writer: Writer,
}

impl Editor {
    /// constructor
    pub fn new() -> Self {
        Self {
            reader: Reader,
            writer: Writer::new(),
        }
    }

    /// process keypress
    pub fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            // press Ctrl-Q to quit
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => return Ok(false),
            // direction controller
            KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End),
                modifiers: KeyModifiers::NONE,
            } => self.writer.move_cursor(direction),
            // PageUp and PageDown
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => (0..self.writer.win_size().1).for_each(|_| {
                self.writer.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),
            _ => {}
        }
        Ok(true)
    }

    /// run the editor
    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.writer.refresh_screen()?;
        self.process_keypress()
    }
}