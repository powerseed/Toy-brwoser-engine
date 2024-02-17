use std::collections::HashMap;
use crate::node::Node;
use crate::node_type::NodeType;
use crate::stylesheet::Stylesheet;

pub struct Parser {
    content: String,
    current_position: usize
}

impl Parser {
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

    pub fn parse_tag_name(&mut self) -> String {
        return self.consume_while(|char| {
            match char {
                'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => true,
                _ => false
            }
        });
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while !self.is_eof() && !self.is_started_with("</") {
            self.consume_whitespaces();
            nodes.push(self.parse_node());
        }

        return nodes;
    }

    pub fn parse_node(&mut self) -> Node {
        if self.peek_current_char() == '<' {
            return self.parse_element()
        }
        else {
            return self.parse_text()
        }
    }

    pub fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut name_value_pairs = HashMap::new();

        while self.peek_current_char() != '>' {
            self.consume_whitespaces();
            let (name, value) = self.parse_attribute();
            name_value_pairs.insert(name, value);
        }

        return name_value_pairs;
    }

    pub fn parse_attribute(&mut self) -> (String, String) {
        println!("{}", self.content[self.current_position..].to_string());
        let name= self.parse_tag_name();
        assert_eq!(self.pop_current_char(), '=');

        let open_quote = self.pop_current_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|char| char != open_quote);
        assert_eq!(self.pop_current_char(), open_quote);

        return (name, value);
    }

    pub fn parse_element(&mut self) -> Node {
        assert_eq!(self.pop_current_char(), '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert_eq!(self.pop_current_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.pop_current_char(), '<');
        assert_eq!(self.pop_current_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.pop_current_char(), '>');

        return Node::new_element(tag_name, attributes, children);
    }

    pub fn parse_text(&mut self) -> Node {
        let text = self.consume_while(|char| char != '<');
        return Node::new_text(text);
    }

    pub fn parse(&mut self) -> Node {
        let mut nodes = self.parse_nodes();
        
        if nodes.len() == 1 {
            return nodes.swap_remove(0);
        }
        else {
            return Node::new_element("html".to_string(), HashMap::new(), nodes);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_is_eof() {
        let parser_eof = Parser {
            content: String::from("asdfghjkl"),
            current_position: 9
        };

        let parser_not_eof = Parser {
            content: String::from("asdfghjkl"),
            current_position: 8
        };

        assert_eq!(true, parser_eof.is_eof());
        assert_eq!(false, parser_not_eof.is_eof());
    }

    #[test]
    fn test_consume_char() {
        let mut parser = Parser {
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
        let mut parser = Parser {
            content: String::from("d    A"),
            current_position: 1
        };
        parser.consume_whitespaces();
        assert_eq!('A', parser.peek_current_char());
    }
}