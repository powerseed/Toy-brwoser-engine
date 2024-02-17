use std::collections::HashMap;
use crate::general_parser::GeneralParser;
use crate::html_parser::node::Node;

mod node;
mod node_type;
mod element_data;

pub struct HTMLParser {
    general_parser: GeneralParser
}

impl HTMLParser {
    pub fn parse_tag_name(&mut self) -> String {
        return self.general_parser.consume_while(|char| {
            match char {
                'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => true,
                _ => false
            }
        });
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while !self.general_parser.is_eof() && !self.general_parser.is_started_with("</") {
            self.general_parser.consume_whitespaces();
            nodes.push(self.parse_node());
        }

        return nodes;
    }

    pub fn parse_node(&mut self) -> Node {
        if self.general_parser.peek_current_char() == '<' {
            return self.parse_element()
        }
        else {
            return self.parse_text()
        }
    }

    pub fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut name_value_pairs = HashMap::new();

        while self.general_parser.peek_current_char() != '>' {
            self.general_parser.consume_whitespaces();
            let (name, value) = self.parse_attribute();
            name_value_pairs.insert(name, value);
        }

        return name_value_pairs;
    }

    pub fn parse_attribute(&mut self) -> (String, String) {
        println!("{}", self.general_parser.content[self.general_parser.current_position..].to_string());
        let name= self.parse_tag_name();
        assert_eq!(self.general_parser.pop_current_char(), '=');

        let open_quote = self.general_parser.pop_current_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.general_parser.consume_while(|char| char != open_quote);
        assert_eq!(self.general_parser.pop_current_char(), open_quote);

        return (name, value);
    }

    pub fn parse_element(&mut self) -> Node {
        assert_eq!(self.general_parser.pop_current_char(), '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert_eq!(self.general_parser.pop_current_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.general_parser.pop_current_char(), '<');
        assert_eq!(self.general_parser.pop_current_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.general_parser.pop_current_char(), '>');

        return Node::new_element(tag_name, attributes, children);
    }

    pub fn parse_text(&mut self) -> Node {
        let text = self.general_parser.consume_while(|char| char != '<');
        return Node::new_text(text);
    }

    pub fn parse(&mut self) -> Node {
        let mut nodes = self.parse_nodes();

        if nodes.len() == 1 {
            return nodes.swap_remove(0);
        }
        else {
            return Node::new_element("html_parser".to_string(), HashMap::new(), nodes);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::general_parser::GeneralParser;
    use crate::html_parser::HTMLParser;

    #[test]
    fn test_parse() {
        let mut parser = HTMLParser {
            general_parser: GeneralParser {
                content: String::from("<html><body><h1>Title</h1><div id='main' class='test'><p>Hello <em>world</em>!</p></div></body></html>"),
                current_position: 0
            }
        };

        let node = parser.parse();
    }
}