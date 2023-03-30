use crate::lexer::Line;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Types {
    String,
    Number,
    Identifier,
    Unknown,
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Label {
    pub label_name: String,
    pub label_code: Vec<(usize, Line)>,
}

#[must_use]
pub fn has_label(labels: Vec<Label>, label_name: String) -> bool {
    for label in &labels {
        if label.label_name == label_name {
            return true;
        }
    }
    return false
}

#[must_use]
pub fn get_code_from(labels: Vec<Label>, label_name: String) -> Vec<(usize, Line)> {
    for label in &labels {
        if label.label_name == label_name {
            return label.label_code.clone();
        }
    }

    return Vec::new()
}
