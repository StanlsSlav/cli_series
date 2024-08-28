use std::{ops::Add, sync::Arc};

use super::UserInput;
use crate::{
    app::App,
    main_key_handler, main_render,
    user::{
        create_input::CreateInput,
        create_series::{create_key_handler, create_render},
        InputType,
    },
};

pub fn move_down(app: &mut App, input: &UserInput) {
    let max_series = app.data.available_series.len() - 1;

    let digits_prefix = input.digits_prefix.unwrap_or(1);

    app.data.hovered_series_idx = app
        .data
        .hovered_series_idx
        .saturating_add(digits_prefix)
        .min(max_series);
}

pub(crate) fn move_up(app: &mut App, input: &UserInput) {
    let digits_prefix = input.digits_prefix.unwrap_or(1);

    app.data.hovered_series_idx = app
        .data
        .hovered_series_idx
        .saturating_sub(digits_prefix)
        .max(0);
}

pub(crate) fn move_min(app: &mut App) {
    app.data.hovered_series_idx = 0;
}

pub(crate) fn move_max(app: &mut App) {
    app.data.hovered_series_idx = app.data.available_series.len() - 1;
}

pub(crate) fn move_to(app: &mut App, input: &UserInput) {
    let max_series = app.data.available_series.len() - 1;
    app.data.hovered_series_idx = input.digits_prefix.unwrap_or(1).clamp(0, max_series);
}

pub(crate) fn scroll_up(app: &mut App, input: &UserInput) {
    if app.data.skip == 0 {
        return;
    }

    let offset = -(input.digits_prefix.unwrap_or(1) as i32);
    scroll(app, offset);
}

pub(crate) fn scroll_down(app: &mut App, input: &UserInput) {
    let offset = input.digits_prefix.unwrap_or(1);
    let window_size = app.data.take + offset;

    if window_size > app.data.total_series {
        return;
    }
    scroll(app, offset.try_into().unwrap());
}

fn scroll(app: &mut App, qty: i32) {
    let new_skip = qty.add(app.data.skip as i32) as usize;
    app.data.skip = new_skip;
    app.data.ignore_cached_series = true;
}

pub(crate) fn start_listing(app: &mut App) {
    app.keyboard_handler = Arc::new(main_key_handler);
    app.renderer = Arc::new(main_render);

    move_min(app);
}

pub(crate) fn start_inserting(app: &mut App) {
    app.keyboard_handler = Arc::new(create_key_handler);
    app.renderer = Arc::new(create_render);

    app.create_data = vec![
        CreateInput::new("Name", InputType::String),
        CreateInput::new("Finished?", InputType::Boolean),
        CreateInput::new("Airing Finished?", InputType::Boolean),
        CreateInput::new("Current Episode", InputType::Number),
        CreateInput::new("Total Episodes", InputType::Number),
    ];

    move_min(app);
}
