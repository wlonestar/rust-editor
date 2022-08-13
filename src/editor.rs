use crate::reader::Reader;
use crate::writer::Writer;
use crate::{prompt, QUIT_TIMES};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cmp;

/// Editor struct
pub struct Editor {
    reader: Reader,
    writer: Writer,
    quit_time: u8,
}

impl Editor {
    /// constructor
    pub fn new() -> Self {
        Self {
            reader: Reader,
            writer: Writer::new(),
            quit_time: QUIT_TIMES,
        }
    }

    /// process keypress
    pub fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            // press Ctrl-Q to quit
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.writer.dirty > 0 && self.quit_time > 0 {
                    self.writer.status_message.set_message(format!(
                        "WARNING!! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_time
                    ));
                    self.quit_time -= 1;
                    return Ok(true);
                }
                return Ok(false);
            }
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
            // Scrolling with PageUp and PageDown
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(val, KeyCode::PageUp) {
                    self.writer.cursor_controller.cursor_y =
                        self.writer.cursor_controller.row_offset;
                } else {
                    self.writer.cursor_controller.cursor_y = cmp::min(
                        self.writer.win_size.1 + self.writer.cursor_controller.row_offset - 1,
                        self.writer.editor_rows.number_of_rows(),
                    );
                }
                (0..self.writer.win_size.1).for_each(|_| {
                    self.writer.move_cursor(if matches!(val, KeyCode::PageUp) {
                        KeyCode::Up
                    } else {
                        KeyCode::Down
                    });
                })
            }
            // save file
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if matches!(self.writer.editor_rows.filename, None) {
                    let prompt = prompt!(&mut self.writer, "Save as: {}").map(|it| it.into());
                    if let None = prompt {
                        self.writer
                            .status_message
                            .set_message("Save Aborted".into());
                        return Ok(true);
                    }
                    self.writer.editor_rows.filename = prompt;
                }
                self.writer.editor_rows.save().map(|len| {
                    self.writer
                        .status_message
                        .set_message(format!("{} bytes written to disk", len));
                    self.writer.dirty = 0;
                })?;
            }
            // delete char
            KeyEvent {
                code: key @ (KeyCode::Backspace | KeyCode::Delete),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(key, KeyCode::Delete) {
                    self.writer.move_cursor(KeyCode::Right);
                }
                self.writer.delete_char();
            }
            // insert new line
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                self.writer.insert_newline();
            }
            // insert char
            KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => self.writer.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(ch) => ch,
                _ => unreachable!(),
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
