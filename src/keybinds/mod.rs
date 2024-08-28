use regex::Regex;

pub(crate) mod nav;

#[derive(PartialEq)]
pub(crate) enum Mode {
    Navigation,
    Edit,
}

pub(crate) struct UserInput {
    pub digits_prefix: Option<usize>,
    pub actions: Vec<char>,
    pub digits_subfix: Option<usize>,
    pub raw_input: Option<String>,
}

pub fn parse_input(input: &str) -> UserInput {
    let mut parsed_input = UserInput {
        digits_prefix: None,
        actions: vec![],
        digits_subfix: None,
        raw_input: Some(input.to_string()),
    };

    let re = Regex::new(r"^(?<digits_prefix>\d*)(?<actions>\D*)(?<digits_subfix>\d*)$").unwrap();
    let captures = re.captures(input).unwrap();

    let digits_prefix = &captures["digits_prefix"];
    let actions = &captures["actions"];
    let digits_subfix = &captures["digits_subfix"];

    if let Ok(nr) = digits_prefix.parse::<usize>() {
        parsed_input.digits_prefix = Some(nr)
    };

    parsed_input.actions = actions.chars().collect();

    if let Ok(nr) = digits_subfix.parse::<usize>() {
        parsed_input.digits_subfix = Some(nr)
    };

    parsed_input
}
