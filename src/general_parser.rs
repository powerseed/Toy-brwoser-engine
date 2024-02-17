pub struct GeneralParser {
    pub content: String,
    pub current_position: usize
}

impl GeneralParser {
    pub fn peek_current_char(&self) -> char {
        return self.content[self.current_position..].chars().next().unwrap()
    }

    pub fn is_started_with(&self, starting_string: &str) -> bool {
        return self.content[self.current_position..].starts_with(starting_string)
    }

    pub fn is_eof(&self) -> bool {
        return self.content[self.current_position..].chars().next().is_none()
    }

    pub fn pop_current_char(&mut self) -> char {
        let current_char = self.peek_current_char();
        self.current_position += current_char.len_utf8();
        return current_char;
    }

    pub fn consume_while<F>(&mut self, checker_function: F) -> String
        where F: Fn(char) -> bool
    {
        let mut result = String::new();

        while !self.is_eof() && checker_function(self.peek_current_char()) {
            result.push(self.pop_current_char());
        }

        return result;
    }

    pub fn consume_whitespaces(&mut self) {
        self.consume_while(|char| char.is_whitespace());
    }
}

#[cfg(test)]
mod tests {
    use crate::general_parser::GeneralParser;

    #[test]
    fn test_is_eof() {
        let parser_eof = GeneralParser {
            content: String::from("asdfghjkl"),
            current_position: 9
        };

        let parser_not_eof = GeneralParser {
            content: String::from("asdfghjkl"),
            current_position: 8
        };

        assert_eq!(true, parser_eof.is_eof());
        assert_eq!(false, parser_not_eof.is_eof());
    }

    #[test]
    fn test_consume_char() {
        let mut parser = GeneralParser {
            content: String::from("dAgßjℝ💣"),
            current_position: 1
        };

        assert_eq!('A', parser.pop_current_char());
        assert_eq!('g', parser.pop_current_char());
        assert_eq!('ß', parser.pop_current_char());
        assert_eq!('j', parser.pop_current_char());
        assert_eq!('ℝ', parser.pop_current_char());
        assert_eq!('💣', parser.pop_current_char());
        assert_eq!(true, parser.is_eof());
    }

    #[test]
    fn test_consume_whitespaces() {
        let mut parser = GeneralParser {
            content: String::from("d    A"),
            current_position: 1
        };
        parser.consume_whitespaces();
        assert_eq!('A', parser.peek_current_char());
    }
}