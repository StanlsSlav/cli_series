use std::char;

pub(crate) mod nav;

pub(crate) enum Mode {
    Navigation,
    Edit,
}

pub(crate) struct UserInput {
    pub digits_prefix: Option<usize>,
    pub actions: Vec<char>,
    pub raw_input: Option<String>,
}

pub fn parse_input(input: &str) -> UserInput {
    let mut parsed_input = UserInput {
        digits_prefix: None,
        actions: vec![],
        raw_input: Some(input.to_string()),
    };

    let mut prefix_digits_consumed = false;

    input.chars().for_each(|c| {
        prefix_digits_consumed = !prefix_digits_consumed && !c.is_ascii_digit();

        match !prefix_digits_consumed {
            true => {
                let digit = c.to_string().parse::<usize>();

                if parsed_input.digits_prefix.is_none() {
                    parsed_input.digits_prefix = Some(digit.unwrap());
                } else {
                    parsed_input.digits_prefix =
                        Some(parsed_input.digits_prefix.unwrap() * 10 + digit.unwrap());
                }
            }
            false => {
                parsed_input.actions.push(c);
            }
        }
    });

    parsed_input
}
