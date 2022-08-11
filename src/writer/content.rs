use std::io::{stdout, ErrorKind, Write};

/// Editor Contents struct
pub struct EditorContents {
    content: String,
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
