use crate::writer::content::{EditorRows, Row};
use crate::TAB_SIZE;
use crossterm::event::KeyCode;
use std::cmp;
use std::cmp::Ordering;

/// Cursor Controller struct
pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub screen_columns: usize,
    pub screen_rows: usize,
    pub row_offset: usize,
    pub column_offset: usize,
    pub render_x: usize,
}

impl CursorController {
    /// constructor
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
            render_x: 0,
        }
    }
}

impl CursorController {
    /// scroll the screen
    pub fn scroll(&mut self, editor_rows: &EditorRows) {
        self.render_x = 0;
        if self.cursor_y < editor_rows.number_of_rows() {
            self.render_x = self.get_render_x(editor_rows.get_editor_row(self.cursor_y))
        }
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        self.column_offset = cmp::min(self.column_offset, self.render_x);
        if self.render_x >= self.column_offset + self.screen_columns {
            self.column_offset = self.render_x - self.screen_columns + 1;
        }
    }

    /// get render_x
    pub fn get_render_x(&self, row: &Row) -> usize {
        row.row_content[..self.cursor_x]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + (TAB_SIZE - 1) - (render_x % TAB_SIZE) + 1
                } else {
                    render_x + 1
                }
            })
    }

    /// move cursor by arrow keys
    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let number_of_rows = editor_rows.number_of_rows();
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            // move left at start of a line - to the end of the previous line
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_render(self.cursor_y).len();
                }
            }
            KeyCode::Down => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1;
                }
            }
            // move right as end of a line - to the beginning of the next line
            KeyCode::Right => {
                if self.cursor_y < number_of_rows {
                    match self
                        .cursor_x
                        .cmp(&editor_rows.get_render(self.cursor_y).len())
                    {
                        Ordering::Less => self.cursor_x += 1,
                        Ordering::Equal => {
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::End => {
                if self.cursor_y < number_of_rows {
                    self.cursor_x = editor_rows.get_render(self.cursor_y).len();
                }
            }
            KeyCode::Home => {
                self.cursor_x = 0;
            }
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < number_of_rows {
            editor_rows.get_render(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_len);
    }
}
