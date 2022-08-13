use crate::writer::highlight::{HighlightType, SyntaxHighlight};
use crate::{Writer, TAB_SIZE};
use std::io::{stdout, Error, ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs, io};

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
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(ErrorKind::WriteZero.into()),
        }
    }

    /// impl flush() - clear the content for the next screen refresh
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

/// Row struct
#[derive(Default)]
pub struct Row {
    pub row_content: String,
    pub render: String,
    pub highlight: Vec<HighlightType>,
    pub is_comment: bool,
}

impl Row {
    /// constructor
    pub fn new(row_content: String, render: String) -> Self {
        Self {
            row_content,
            render,
            highlight: Vec::new(),
            is_comment: false,
        }
    }

    /// insert char
    pub fn insert_char(&mut self, at: usize, ch: char) {
        self.row_content.insert(at, ch);
        EditorRows::render_row(self);
    }

    /// delete char
    pub fn delete_char(&mut self, at: usize) {
        self.row_content.remove(at);
        EditorRows::render_row(self)
    }

    pub fn get_row_content_x(&self, render_x: usize) -> usize {
        let mut current_render_x = 0;
        for (cursor_x, ch) in self.row_content.chars().enumerate() {
            if ch == '\t' {
                current_render_x += (TAB_SIZE - 1) - (current_render_x % TAB_SIZE);
            }
            current_render_x += 1;
            if current_render_x > render_x {
                return cursor_x;
            }
        }
        0
    }
}

/// Editor Rows struct
pub struct EditorRows {
    pub row_contents: Vec<Row>,
    pub filename: Option<PathBuf>,
}

impl EditorRows {
    /// constructor
    pub fn new(syntax_highlight: &mut Option<Box<dyn SyntaxHighlight>>) -> Self {
        match env::args().nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into(), syntax_highlight),
        }
    }

    /// display from file
    pub fn from_file(
        file: PathBuf,
        syntax_highlight: &mut Option<Box<dyn SyntaxHighlight>>,
    ) -> Self {
        let file_contents = fs::read_to_string(&file).expect("Unable to read file");
        let mut row_contents = Vec::new();
        file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| Writer::select_syntax(ext).map(|syntax| syntax_highlight.insert(syntax)));
        file_contents.lines().enumerate().for_each(|(i, line)| {
            let mut row = Row::new(line.into(), String::new());
            Self::render_row(&mut row);
            row_contents.push(row);
            if let Some(it) = syntax_highlight {
                it.update_syntax(i, &mut row_contents)
            }
        });
        Self {
            row_contents,
            filename: Some(file),
        }
    }

    /// save to the disk
    pub fn save(&self) -> io::Result<usize> {
        match &self.filename {
            None => Err(Error::new(ErrorKind::Other, "no file name specified")),
            Some(name) => {
                let mut file = fs::OpenOptions::new().write(true).create(true).open(name)?;
                let contents: String = self
                    .row_contents
                    .iter()
                    .map(|it| it.row_content.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n");
                file.set_len(contents.len() as u64)?;
                file.write_all(contents.as_bytes())?;
                Ok(contents.as_bytes().len())
            }
        }
    }

    /// rows number
    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    /// get the at render
    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    /// get the at editor row
    pub fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    /// get the at editor row as mut
    pub fn get_editor_row_mut(&mut self, at: usize) -> &mut Row {
        &mut self.row_contents[at]
    }

    /// insert row
    pub fn insert_row(&mut self, at: usize, contents: String) {
        let mut new_row = Row::new(contents, String::new());
        EditorRows::render_row(&mut new_row);
        self.row_contents.insert(at, new_row);
    }

    /// backspacing at the start of the line
    pub fn join_adjacent_rows(&mut self, at: usize) {
        let current_row = self.row_contents.remove(at);
        let previous_row = self.get_editor_row_mut(at - 1);
        previous_row.row_content.push_str(&current_row.row_content);
        Self::render_row(previous_row);
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
