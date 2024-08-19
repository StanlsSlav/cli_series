use uuid::Uuid;

use crate::{app::App, input, keybinds::{nav::{move_down, move_max, move_min, move_to, move_up}, parse_input}, term::clear_screen, Mode};

use super::InputType;
use std::{
    io::{self},
    option::Option,
};

#[derive(Clone)]
enum DefaultValue {
    Text(String),
    Get(fn() -> String),
}

impl DefaultValue {
    fn get(&self) -> String {
        match self {
            DefaultValue::Text(text) => text.clone(),
            DefaultValue::Get(get) => get(),
        }
    }
}

pub(crate) struct CreateInput {
    pub(crate) label: String,
    pub(crate) input_type: InputType,
    pub(crate) value: Option<DefaultValue>,
}

impl CreateInput {
    pub(crate) fn new(label: &str, input_type: InputType) -> Self {
        let label = label.to_string();

        Self {
            label,
            input_type,
            value: None,
        }
    }
}

pub(crate) fn begin(app: &mut App) -> Result<(), io::Error> {
    let inputs = vec![
        CreateInput {
            label: "Guid".to_string(),
            input_type: InputType::String,
            value: Some(DefaultValue::Get(|| Uuid::new_v4().to_string())),
        },
        CreateInput::new("Name", InputType::String),
        CreateInput::new("Finished?", InputType::Boolean),
        CreateInput::new("Airing Finished?", InputType::Boolean),
        CreateInput::new("Current Episode", InputType::Number),
        CreateInput::new("Total Episodes", InputType::Number),
    ];

    loop {
        if app.should_exit {
            break;
        }

        if app.should_render {
            render(&inputs);
            app.should_render = false;
        }
    }

    Ok(())
}

pub(crate) fn render(inputs: &[CreateInput]) {
    clear_screen();

    let mut form_inputs: Vec<(String, String)> = vec![];
    for input in inputs {
        let value = match &input.value {
            Some(default_value) => default_value.get(),
            None => "".to_string(),
        };

        form_inputs.push((input.label.clone(), value));
    }
}

pub(crate) fn create_key_handler(app: &mut App) {
    let user_input = input::get();
    let input = parse_input(user_input.trim());

    let raw_input = input.raw_input.clone().unwrap();
    let raw_input = raw_input.as_str();

    if raw_input.ends_with("k") {
        move_up(app, &input);
    } else if raw_input.ends_with("j") {
        move_down(app, &input);
    } else if raw_input == "G" {
        move_max(app);
    } else if raw_input == "gg" {
        move_min(app);
    } else if raw_input != "0" && raw_input.chars().all(|x| x.is_ascii_digit()) {
        move_to(app, &input);
    }

    match raw_input {
        "n" => app.mode = Mode::Navigation,
        "e" => app.mode = Mode::Edit,
        _ => {}
    }
}
