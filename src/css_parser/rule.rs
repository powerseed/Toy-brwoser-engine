use crate::css_parser::declaration::Declaration;
use crate::css_parser::selector::Selector;

pub struct Rule {
    selector: Vec<Selector>,
    declaration: Vec<Declaration>
}