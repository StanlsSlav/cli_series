use std::sync::Arc;

use super::UserInput;
use crate::{
    app::App,
    user::{self, create_series::create_key_handler},
};

pub fn move_down(app: &mut App, input: &UserInput) {
    let max_series = app.data.available_series.len() - 1;

    let digits_prefix = input.digits_prefix.unwrap_or(1usize);

    if let Some(idx) = app.data.hovered_series_idx {
        app.data.hovered_series_idx = Some(idx.saturating_add(digits_prefix).min(max_series));
    }
}

pub(crate) fn move_up(app: &mut App, input: &UserInput) {
    let digits_prefix = input.digits_prefix.unwrap_or(1);

    if let Some(idx) = app.data.hovered_series_idx {
        app.data.hovered_series_idx = Some(idx.saturating_sub(digits_prefix).max(0));
    }
}

pub(crate) fn move_min(app: &mut App) {
    app.data.hovered_series_idx = Some(0);
}

pub(crate) fn move_max(app: &mut App) {
    app.data.hovered_series_idx = Some(app.data.available_series.len() - 1);
}

pub(crate) fn move_to(app: &mut App, input: &UserInput) {
    let max_series = app.data.available_series.len() - 1;
    app.data.hovered_series_idx = Some(input.digits_prefix.unwrap_or(1).clamp(0, max_series));
}

pub(crate) fn start_inserting(app: &mut App) {
    app.keyboard_handler = Arc::new(create_key_handler);
    let _ = user::create_series::begin(app);
}
