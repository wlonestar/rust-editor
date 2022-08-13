pub mod content;
pub mod cursor_controller;
pub mod status;

use crate::writer::content::{EditorContents, EditorRows};
use crate::writer::cursor_controller::CursorController;
use crate::writer::status::StatusMessage;
use crate::VERSION;
use crossterm::event::KeyCode;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, style, terminal};
use std::cmp;
use std::io::{stdout, Write};

/// Writer struct
pub struct Writer {
    pub win_size: (usize, usize),
    pub editor_contents: EditorContents,
    pub cursor_controller: CursorController,
    pub editor_rows: EditorRows,
    pub status_message: StatusMessage,
    pub dirty: u64,
}

impl Writer {
    /// constructor
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 2))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            editor_rows: EditorRows::new(),
            status_message: StatusMessage::new("HELP: Ctrl-S = Save | Ctrl-Q = Quit".into()),
            dirty: 0,
        }
    }

    /// clear screen
    pub fn clear_screen() -> crossterm::Result<()> {
        // clear screen
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        // move cursor to (0, 0) - left over the screen
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// draw welcome message
    fn draw_welcome(&mut self, screen_columns: usize, welcome: &mut String) {
        if welcome.len() > screen_columns {
            welcome.truncate(screen_columns)
        }
        let mut padding = (screen_columns - welcome.len()) / 2;
        if padding != 0 {
            self.editor_contents.push('~');
            padding -= 1;
        }
        (0..padding).for_each(|_| self.editor_contents.push(' '));
        self.editor_contents.push_str(&welcome);
    }

    /// draw editor rows
    pub fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.row_offset;
            if file_row >= self.editor_rows.number_of_rows() {
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    let mut welcome = format!("RIM Editor --- Version {}", VERSION);
                    self.draw_welcome(screen_columns, &mut welcome);
                } else {
                    self.editor_contents.push('~');
                }
            } else {
                let row = self.editor_rows.get_render(file_row);
                let column_offset = self.cursor_controller.column_offset;
                let len = cmp::min(row.len().saturating_sub(column_offset), screen_columns);
                let start = if len == 0 { 0 } else { column_offset };
                self.editor_contents.push_str(&row[start..start + len])
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    /// draw status bar
    pub fn draw_status_bar(&mut self) {
        self.editor_contents
            .push_str(&style::Attribute::Reverse.to_string());
        let info = format!(
            "{} {} - {} lines",
            self.editor_rows
                .filename
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or("[No Name]"),
            if self.dirty > 0 { "(modified)" } else { "" },
            self.editor_rows.number_of_rows()
        );
        let info_len = cmp::min(info.len(), self.win_size.0);
        let line_info = format!(
            "{}/{}",
            self.cursor_controller.cursor_y + 1,
            self.editor_rows.number_of_rows()
        );
        self.editor_contents.push_str(&info[..info_len]);
        for i in info_len..self.win_size.0 {
            if self.win_size.0 - i == line_info.len() {
                self.editor_contents.push_str(&line_info);
                break;
            } else {
                self.editor_contents.push(' ');
            }
        }
        self.editor_contents
            .push_str(&style::Attribute::Reset.to_string());
        self.editor_contents.push_str("\r\n");
    }

    /// draw message bar
    pub fn draw_message_bar(&mut self) {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::UntilNewLine)
        )
        .unwrap();
        if let Some(msg) = self.status_message.message() {
            self.editor_contents
                .push_str(&msg[..cmp::min(self.win_size.0, msg.len())]);
        }
    }

    /// move cursor by arrow keys
    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows);
    }

    /// insert char
    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_controller.cursor_y == self.editor_rows.number_of_rows() {
            self.editor_rows
                .insert_row(self.editor_rows.number_of_rows(), String::new());
            self.dirty += 1;
        }
        self.editor_rows
            .get_editor_row_mut(self.cursor_controller.cursor_y)
            .insert_char(self.cursor_controller.cursor_x, ch);
        self.cursor_controller.cursor_x += 1;
        self.dirty += 1;
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_controller.cursor_x == 0 {
            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y, String::new())
        } else {
            let current_row = self
                .editor_rows
                .get_editor_row_mut(self.cursor_controller.cursor_y);
            let new_row_content = current_row.row_content[self.cursor_controller.cursor_x..].into();
            current_row
                .row_content
                .truncate(self.cursor_controller.cursor_x);
            EditorRows::render_row(current_row);
            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y + 1, new_row_content);
        }
        self.cursor_controller.cursor_x = 0;
        self.cursor_controller.cursor_y += 1;
        self.dirty += 1;
    }

    /// delete char
    pub fn delete_char(&mut self) {
        if self.cursor_controller.cursor_y == self.editor_rows.number_of_rows() {
            return;
        }
        if self.cursor_controller.cursor_y == 0 && self.cursor_controller.cursor_x == 0 {
            return;
        }
        let row = self
            .editor_rows
            .get_editor_row_mut(self.cursor_controller.cursor_y);
        if self.cursor_controller.cursor_x > 0 {
            row.delete_char(self.cursor_controller.cursor_x - 1);
            self.cursor_controller.cursor_x -= 1;
            self.dirty += 1;
        } else {
            let previous_row_content = self
                .editor_rows
                .get_render(self.cursor_controller.cursor_y - 1);
            self.cursor_controller.cursor_x = previous_row_content.len();
            self.editor_rows
                .join_adjacent_rows(self.cursor_controller.cursor_y);
            self.cursor_controller.cursor_y -= 1;
        }
        self.dirty += 1;
    }

    /// refresh screen
    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        self.cursor_controller.scroll(&self.editor_rows);
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        self.draw_status_bar();
        self.draw_message_bar();
        let cursor_x = self.cursor_controller.render_x - self.cursor_controller.column_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
}

#[macro_export]
macro_rules! prompt {
    ($writer:expr, $($args:tt)*) => {{
        let writer:&mut Writer = &mut $writer;
        let mut input = String::with_capacity(32);
        loop {
            writer.status_message.set_message(format!($($args)*, input));
            writer.refresh_screen()?;
            match Reader.read_key()? {
                // confirm
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                } => {
                    if !input.is_empty() {
                        writer.status_message.set_message(String::new());
                        break;
                    }
                }
                // abort
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                } => {
                    if !input.is_empty() {
                        writer.status_message.set_message(String::new());
                        input.clear();
                        break;
                    }
                }
                // delete
                KeyEvent {
                    code: KeyCode::Backspace | KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                } => {
                    input.pop();
                }
                // input
                KeyEvent {
                    code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                } => {
                    input.push(match code {
                        KeyCode::Tab => '\t',
                        KeyCode::Char(ch) => ch,
                        _ => unreachable!(),
                    })
                }
                _ => {}
            }
        }
        if input.is_empty() {
            None
        } else {
            Some(input)
        }
    }};
}
