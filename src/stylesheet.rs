pub struct Stylesheet {
    rules: Vec<Rule>
}

struct Rule {
    selector: Vec<Selector>,
    declaration: Vec<Declaration>
}

struct Selector {
    tag_name: Option<String>,
    id: Option<String>,
    classes: Vec<String>
}

struct Declaration {
    name: String,
    value: DeclarationValue
}

enum DeclarationValue {
    Keyword(String),
    Length(f32, LengthUnit),
    Color(ColorValue)
}

enum LengthUnit {
    Px
}

struct ColorValue {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

