use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ParseError {
    pub line: String,
    pub reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    pub lines: Vec<String>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn from(other: &Paragraph) -> Self {
        return Self {
            lines: other.lines.to_vec(),
        };
    }

    pub fn add_line(&mut self, line: &str) -> Result<(), ()> {
        if line.len() == 0 {
            return Err(());
        }

        self.lines.push(String::from(line));
        return Ok(());
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, num: usize) -> Option<&String> {
        self.lines.get(num)
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }
}
