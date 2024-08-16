use crate::term::{attribute, color};
use app::{App, Data};
use color::Color;
use mode::Mode;
use printer::print_series_table;
use series::Series;
use std::sync::Arc;
use term::clear_screen;
use user::create_series::create_key_handler;

mod app;
mod db;
mod input_handler;
mod mode;
mod printer;
mod series;
mod term;
mod user;

fn main() {
    let mut app = App {
        should_render: true,
        should_exit: false,
        should_show_help: false,
        should_show_help_msg: true,
        keyboard_handler: Arc::new(main_key_handler),
        term_size: termsize::get().unwrap(),
        mode: Mode::Navigation,
        data: Data {
            hovered_series_idx: Some(0),
            available_series: Vec::new(),
            ignore_cached_series: false,
            take: Some(16),
            skip: None,
        },
    };

    while !app.should_exit {
        let sizes = termsize::get().unwrap();
        let has_term_resized = app.term_size.cols != sizes.cols || app.term_size.rows != sizes.rows;

        if app.should_render || !has_term_resized {
            app.term_size = sizes;

            render(&mut app);
            app.should_render = false;
        }

        handle_input(&mut app);
    }

    clear_screen();
}

fn print_help() {
    let keybinds = [
        ('h', "Toggle Help"),
        ('m', "Toggle help Message"),
        ('r', "Force Refresh the series"),
        ('k', "Move up"),
        ('j', "Move down"),
        ('G', "Move to last series"),
    ];

    for (key, desc) in keybinds.iter() {
        println!(
            "{}{}{:?}: {}",
            Color::Magenta,
            key,
            attribute::reset(),
            desc
        );
    }
}

fn render(app: &mut App) {
    clear_screen();

    if app.should_show_help {
        print_help();
        return;
    }

    let data = &mut app.data;

    if data.available_series.is_empty() || data.ignore_cached_series {
        let series = Series::get(data.take, data.skip).unwrap_or_else(|_| vec![]);
        data.available_series.clone_from(&series);

        data.ignore_cached_series = false;
    }

    print_series_table(&data.available_series, data.hovered_series_idx);
    println!();

    if app.should_show_help_msg {
        println!("Press [h] for keybinds");
    }
}

fn handle_input(app: &mut App) {
    app.keyboard_handler.clone()(app);
}

fn main_key_handler(app: &mut App) {
    app.should_render = true;

    let binding = input_handler::get_input();
    let input = binding.trim();

    app.should_exit = input == "q";
    app.should_show_help = input == "h" && !app.should_show_help;
    app.should_show_help_msg = input == "m" && !app.should_show_help_msg;
    app.data.ignore_cached_series = input == "r";

    let found_digits_prefix: Vec<usize> = input
        .matches("^\\d+")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap_or(1))
        .collect();

    let digits_prefix = found_digits_prefix.first().unwrap_or(&1);

    if input == "i" {
        app.keyboard_handler = Arc::new(create_key_handler);
        let _ = user::create_series::begin(app);
    } else if input.ends_with("k") {
        if let Some(idx) = app.data.hovered_series_idx {
            app.data.hovered_series_idx = Some(idx.saturating_sub(*digits_prefix).max(0));
        }
    } else if input.ends_with("j") {
        let max_series = app.data.available_series.len() - 1;

        if let Some(idx) = app.data.hovered_series_idx {
            app.data.hovered_series_idx = Some(idx.saturating_add(*digits_prefix).min(max_series));
        }
    } else if input == "G" {
        app.data.hovered_series_idx = Some(app.data.available_series.len() - 1);
    } else if input != "0" && input.chars().all(|x| x.is_ascii_digit()) {
        let max_series = app.data.available_series.len() - 1;
        let idx = input.parse::<usize>().unwrap_or(0) - 1;
        app.data.hovered_series_idx = Some(idx.clamp(0, max_series));
    }
}
