use crate::core::base::Paragraph;
use std::collections::VecDeque;

pub struct CommitMessage {
    pub paragraphs: Vec<Paragraph>,
}

impl CommitMessage {
    pub fn from(file_content: &str) -> Self {
        let mut paragraphs: Vec<Paragraph> = vec![];

        let mut current_paragraph: Paragraph = Paragraph::new();
        for line in file_content.lines() {
            let trimmed_line = line.trim();
            if Self::is_message_line(trimmed_line) {
                current_paragraph
                    .add_line(trimmed_line)
                    .expect("Failed to add line to paragraph")
            } else {
                if current_paragraph.len() > 0 {
                    paragraphs.push(current_paragraph);
                }
                current_paragraph = Paragraph::new()
            }
        }

        if current_paragraph.len() > 0 {
            paragraphs.push(current_paragraph);
        }

        return CommitMessage { paragraphs };
    }

    pub fn get_paragraphs(&self) -> VecDeque<Paragraph> {
        let mut deque: VecDeque<Paragraph> = VecDeque::with_capacity(self.paragraphs.len());
        for par in self.paragraphs.iter() {
            deque.push_back(Paragraph::from(par));
        }

        return deque;
    }

    pub fn get_paragraph(self: &Self, num: usize) -> Option<&Paragraph> {
        if num >= self.paragraphs.len() {
            return None;
        }

        let ref paragraph = self.paragraphs[num];
        return Some(paragraph);
    }

    fn is_message_line(line: &str) -> bool {
        if line.len() == 0 {
            return false;
        }

        if line.starts_with("#") {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commit_message::CommitMessage;

    #[test]
    fn should_create_commit_message_with_separate_three_paragraphs() {
        // given
        let input_string = r"first line of 1st paragraph

first line of 2nd paragraph
second line of 2nd paragraph

first line of 3rd paragraph


first line of 4th paragraph
        ";

        // when
        let commit_message = CommitMessage::from(input_string);

        // then
        assert_eq!(
            commit_message.paragraphs[0].lines,
            vec![String::from("first line of 1st paragraph")]
        );
        assert_eq!(
            commit_message.paragraphs[1].lines,
            vec![
                String::from("first line of 2nd paragraph"),
                String::from("second line of 2nd paragraph")
            ]
        );
        assert_eq!(
            commit_message.paragraphs[2].lines,
            vec![String::from("first line of 3rd paragraph")]
        );
        assert_eq!(
            commit_message.paragraphs[3].lines,
            vec![String::from("first line of 4th paragraph")]
        );
    }

    #[test]
    fn should_skip_lines_staring_with_hash_character() {
        // given
        let input_string = r"#foo
        bar
        #baz

        #foo1
        bar1
        baz1
        #
        foo2
        ";

        // when
        let commit_msg = CommitMessage::from(input_string);

        // then
        assert_eq!(commit_msg.paragraphs[0].lines, vec![String::from("bar")]);
        assert_eq!(
            commit_msg.paragraphs[1].lines,
            vec![String::from("bar1"), String::from("baz1"),]
        );
        assert_eq!(commit_msg.paragraphs[2].lines, vec![String::from("foo2")]);
    }
}
