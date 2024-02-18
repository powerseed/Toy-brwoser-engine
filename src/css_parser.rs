use crate::css_parser::color_value::ColorValue;
use crate::css_parser::declaration::Declaration;
use crate::css_parser::declaration_value::DeclarationValue;
use crate::css_parser::length_unit::LengthUnit;
use crate::css_parser::rule::Rule;
use crate::css_parser::selector::Selector;
use crate::css_parser::stylesheet::Stylesheet;
use crate::general_parser::GeneralParser;

mod stylesheet;
mod rule;
mod selector;
mod declaration;
mod declaration_value;
mod length_unit;
mod color_value;

pub struct CSSParser {
    general_parser: GeneralParser
}

impl CSSParser {
    pub fn parse(&mut self) -> Stylesheet {
        Stylesheet {
            rules: self.parser_rules()
        }
    }

    pub fn parser_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();

        while !self.general_parser.is_eof() {
            self.general_parser.consume_whitespaces();
            let rule = self.parse_rule();
            rules.push(rule);
        }

        return rules;
    }

    pub fn parse_rule(&mut self) -> Rule {
        let selectors = self.parse_selectors();
        let declarations = self.parse_declarations();

        Rule {
            selectors,
            declarations
        }
    }

    pub fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(self.parse_selector());
            self.general_parser.consume_whitespaces();

            match self.general_parser.peek_current_char() {
                ',' => {
                    self.general_parser.pop_current_char();
                    self.general_parser.consume_whitespaces();
                },
                '{' => {
                    break;
                },
                c => {
                    println!("Unexpected character in CSS selector: {}. ", c);
                    panic!("Unexpected character in CSS selector: {}. ", c)
                }
            }
        }

        selectors.sort_by(|a ,b| {
            a.get_specificity().cmp(&b.get_specificity())
        });

        return selectors;
    }

    pub fn parse_selector(&mut self) -> Selector {
        let mut selector = Selector {
            tag_name: None,
            id: None,
            classes: Vec::new()
        };

        while !self.general_parser.is_eof() {
            match self.general_parser.peek_current_char() {
                // Next is class name
                '.' => {
                    self.general_parser.pop_current_char();
                    selector.classes.push(self.general_parser.consume_while(is_valid_identifier_char));
                },
                // Next is id
                '#' => {
                    self.general_parser.pop_current_char();
                    selector.id = Some(self.general_parser.consume_while(is_valid_identifier_char));
                },
                // Next is universal identifier
                '*' => {
                    self.general_parser.pop_current_char();
                },
                // Next is tag name
                c if is_valid_identifier_char(c) => {
                    selector.tag_name = Some(self.general_parser.consume_while(is_valid_identifier_char));
                },
                _ => break
            }
        }

        return selector;
    }

    pub fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        assert_eq!('{', self.general_parser.pop_current_char());
        self.general_parser.consume_whitespaces();

        while !self.general_parser.is_eof() {
            declarations.push(self.parse_declaration());
            self.general_parser.consume_whitespaces();

            if self.general_parser.peek_current_char() == '}' {
                self.general_parser.pop_current_char();
                break;
            }
        }

        return declarations;
    }

    pub fn parse_declaration(&mut self) -> Declaration {
        let name = self.general_parser.consume_while(is_valid_identifier_char);
        self.general_parser.consume_whitespaces();
        assert_eq!(':', self.general_parser.pop_current_char());
        self.general_parser.consume_whitespaces();
        let value = self.parse_declaration_value();
        assert_eq!(';', self.general_parser.pop_current_char());

        return Declaration {
            name,
            value
        };
    }

    pub fn parse_declaration_value(&mut self) -> DeclarationValue {
        return match self.general_parser.peek_current_char() {
            '#' => self.parse_color(),
            '0'..='9' => self.parse_length(),
            _ => self.parse_keyword()
        }
    }

    pub fn parse_color(&mut self) -> DeclarationValue {
        assert_eq!('#', self.general_parser.pop_current_char());
        let hex_value = self.general_parser.consume_while(is_valid_identifier_char);
        let (r, g, b) = convert_hex_to_rgb(hex_value.as_str());
        return DeclarationValue::Color(ColorValue { r, g, b, a: 255 });
    }

    pub fn parse_length(&mut self) -> DeclarationValue {
        let number = self.general_parser.consume_while(|c| {
            match c {
                '0' ..= '9' => true,
                _ => false
            }
        });
        let number= number.parse::<f32>().unwrap();
        let unit = self.general_parser.consume_while(is_valid_identifier_char).to_ascii_lowercase();
        let unit = if unit == "px" { LengthUnit::Px } else { panic!() };

        return DeclarationValue::Length(number, unit);
    }

    pub fn parse_keyword(&mut self) -> DeclarationValue {
        let keyword = self.general_parser.consume_while(is_valid_identifier_char);
        return DeclarationValue::Keyword(keyword);
    }
}

fn is_valid_identifier_char(char: char) -> bool {
    match char {
        '0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '-' => true,
        _ => false
    }
}

fn convert_hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    (u8::from_str_radix(&hex[0 .. 2], 16).unwrap(),
     u8::from_str_radix(&hex[2 .. 4], 16).unwrap(),
     u8::from_str_radix(&hex[4 .. 6], 16).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::css_parser::{convert_hex_to_rgb, CSSParser};
    use crate::general_parser::GeneralParser;

    #[test]
    fn test_convert_hex_to_rgb() {
        assert_eq!((250, 128, 114), convert_hex_to_rgb("fa8072"));
    }

    #[test]
    fn test_parse() {
        let mut css_parser = CSSParser {
            general_parser: GeneralParser {
                content: "h1, h2, h3 { margin: auto; color: #cc0000; } div.note { margin-bottom: 20px; padding: 10px; } #answer { display: none; }".parse().unwrap(),
                current_position: 0
            }
        };

        let stylesheet = css_parser.parse();
        assert_eq!("h1, h2, h3 { margin: auto; color: #rgba(204, 0, 0, 255); }
div.note { margin-bottom: 20px; padding: 10px; }
#answer { display: none; }", format!("{}", stylesheet));
    }
}