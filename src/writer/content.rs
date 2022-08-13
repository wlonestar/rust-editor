use crate::TAB_SIZE;
use std::io::{stdout, ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs};

/// Editor Contents struct
pub struct EditorContents {
    pub content: String,
}

impl EditorContents {
    /// constructor
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// push char
    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    /// push string
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl Write for EditorContents {
    /// impl write() - convert the bytes to str -> add it to the content
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(ErrorKind::WriteZero.into()),
        }
    }

    /// impl flush() - clear the content for the next screen refresh
    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

/// Row struct
pub struct Row {
    pub row_content: Box<str>,
    pub render: String,
}

impl Row {
    /// constructor
    pub fn new(row_content: Box<str>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
}

/// Editor Rows struct
pub struct EditorRows {
    pub row_contents: Vec<Row>,
    pub filename: Option<PathBuf>,
}

impl EditorRows {
    /// constructor
    pub fn new() -> Self {
        match env::args().nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into()),
        }
    }
}

impl EditorRows {
    /// display from file
    pub fn from_file(file: PathBuf) -> Self {
        let file_contents = fs::read_to_string(&file).expect("Unable to read file");
        Self {
            row_contents: file_contents
                .lines()
                .map(|it| {
                    let mut row = Row::new(it.into(), String::new());
                    Self::render_row(&mut row);
                    row
                })
                .collect(),
            filename: Some(file),
        }
    }

    /// rows number
    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    /// get the at render
    /// * `at` - the line of row
    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    /// get the at editor row
    /// * `at` - the line of row
    pub fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    /// render row
    pub fn render_row(row: &mut Row) {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_SIZE } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_SIZE != 0 {
                    row.render.push(' ');
                    index += 1
                }
            } else {
                row.render.push(c);
            }
        })
    }
}
