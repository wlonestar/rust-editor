pub mod content;
pub mod cursor_controller;

use crate::writer::content::EditorContents;
use crate::writer::cursor_controller::CursorController;
use crate::VERSION;
use crossterm::event::KeyCode;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, terminal};
use std::io::{stdout, Write};

/// Writer struct
pub struct Writer {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

/// Constructor and Getter
impl Writer {
    /// constructor
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
        }
    }

    pub fn win_size(&self) -> (usize, usize) {
        self.win_size
    }
}

impl Writer {
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
            if i == screen_rows / 3 {
                let mut welcome = format!("RIM Editor --- Version {}", VERSION);
                self.draw_welcome(screen_columns, &mut welcome);
            } else {
                self.editor_contents.push('~');
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

    /// move cursor by arrow keys
    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

    /// refresh screen
    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x();
        let cursor_y = self.cursor_controller.cursor_y();
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
}
