use std::fmt::Display;

pub fn stringify(displayable: Vec<impl Display>) -> String {
    displayable
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join("")
}
