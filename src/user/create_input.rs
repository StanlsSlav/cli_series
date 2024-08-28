use super::InputType;

pub(crate) struct CreateInput {
    pub(crate) label: String,
    pub(crate) input_type: InputType,
    pub(crate) raw_value: String,
}

impl CreateInput {
    pub(crate) fn new(label: &str, input_type: InputType) -> Self {
        let label = label.to_string();

        let value = match input_type {
            InputType::Number => "0",
            InputType::String => "",
            InputType::Boolean => "false",
        };

        Self {
            label,
            input_type,
            raw_value: value.to_string(),
        }
    }

    pub(crate) fn get_i32(&self) -> Option<i32> {
        if self.input_type != InputType::Number {
           return None; 
        }

        match self.raw_value.parse::<i32>() {
            Ok(nr) => Some(nr),
            Err(_) => None,
        }
    }

    pub(crate) fn get_bool(&self) -> Option<bool> {
        match self.input_type {
            InputType::Boolean => Some(self.raw_value.to_ascii_lowercase() == "yes"),
            _ => None,
        }
    }
}
