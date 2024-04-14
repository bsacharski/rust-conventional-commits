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

    pub fn folded(&self) -> Paragraph {
        let mut folded_lines: Vec<String> = vec![];

        if self.lines.len() > 0 {
            let mut lines_iterator = self.lines.iter();

            let mut current_line = String::from(lines_iterator.next().unwrap());
            loop {
                let next_line = lines_iterator.next();

                if next_line.is_none() {
                    folded_lines.push(current_line);
                    break;
                }

                if next_line.unwrap().starts_with(" ") {
                    current_line.push_str(" ");
                    current_line.push_str(next_line.unwrap().trim_start());
                } else {
                    folded_lines.push(current_line);
                    current_line = String::from(next_line.unwrap());
                }
            }
        }

        return Paragraph {
            lines: folded_lines,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::core::base::Paragraph;

    #[test]
    fn should_fold_line_starting_with_trailing_space_into_previous_line() {
        // given
        let lines = vec![
            String::from("first line"),
            String::from("  has continuation"),
            String::from("  that spans over"),
            String::from("   multiple lines"),
        ];

        let paragraph = Paragraph { lines };

        // when
        let folded_paragraph = paragraph.folded();

        // then
        assert_eq!(
            folded_paragraph,
            Paragraph {
                lines: vec![String::from(
                    "first line has continuation that spans over multiple lines"
                )]
            }
        )
    }

    #[test]
    fn should_fold_lines_starting_with_trailing_space_in_a_paragraph_with_multiple_lines_being_folded(
    ) {
        let lines = vec![
            String::from("first line"),
            String::from(" with fold"),
            String::from("second line without a fold"),
            String::from("third line"),
            String::from("  with multiple"),
            String::from(" folds"),
            String::from("fourth line"),
        ];

        let paragraph = Paragraph { lines };

        // when
        let folded_paragraph = paragraph.folded();

        // then
        assert_eq!(
            folded_paragraph,
            Paragraph {
                lines: vec![
                    String::from("first line with fold"),
                    String::from("second line without a fold"),
                    String::from("third line with multiple folds"),
                    String::from("fourth line")
                ]
            }
        )
    }
}
