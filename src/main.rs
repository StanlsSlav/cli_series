use crate::term::{attribute, color};
use app::{App, Data};
use color::Color;
use keybinds::{
    nav::{move_down, move_max, move_min, move_to, move_up, start_inserting},
    parse_input, Mode,
};
use printer::print_series_table;
use series::Series;
use std::sync::Arc;
use term::clear_screen;
use user::create_series::create_key_handler;

mod app;
mod db;
mod input;
mod keybinds;
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

    let user_input = input::get();
    let input = parse_input(user_input.trim());

    let raw_input = input.raw_input.clone().unwrap();
    let raw_input = raw_input.as_str();

    app.should_exit = raw_input == "q";
    app.should_show_help = raw_input == "h" && !app.should_show_help;
    app.should_show_help_msg = raw_input == "m" && !app.should_show_help_msg;
    app.data.ignore_cached_series = raw_input == "r";

    if raw_input == "i" {
        start_inserting(app);
    } else if raw_input.ends_with("k") {
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
}
