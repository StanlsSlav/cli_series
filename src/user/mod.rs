pub(crate) mod create_series;
pub(crate) mod create_input;

#[derive(PartialEq)]
pub(crate) enum InputType {
    Number,
    String,
    Boolean,
}
