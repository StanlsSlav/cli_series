use uuid::Uuid;

use crate::{app::App, input_handler, mode::Mode, term::clear_screen};

use super::input_types::InputType;
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

pub(crate) fn create_key_handler(_app: &mut App) {
    let input: String = input_handler::get_input();

    match input.as_str() {
        "n" => _app.mode = Mode::Navigation,
        "e" => _app.mode = Mode::Edit,
        _ => {}
    }
}
