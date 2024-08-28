use crate::term::{attribute, color};
use app::{App, Data};
use color::Color;
use keybinds::{
    nav::{
        move_down, move_max, move_min, move_to, move_up, scroll_down, scroll_up, start_inserting,
    },
    parse_input, Mode,
};
use printer::print_series_table;
use series::Series;
use std::sync::Arc;
use term::clear_screen;

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
        renderer: Arc::new(main_render),
        term_size: termsize::get().unwrap(),
        mode: Mode::Navigation,
        toast: None,
        data: Data {
            hovered_series_idx: 0,
            available_series: Vec::new(),
            ignore_cached_series: false,
            take: 16,
            skip: 0,
            total_series: Series::count_total().unwrap_or(0),
        },
        create_data: vec![],
    };

    while !app.should_exit {
        let sizes = termsize::get().unwrap();
        let has_term_resized = app.term_size.cols != sizes.cols || app.term_size.rows != sizes.rows;

        if app.should_render || !has_term_resized {
            app.term_size = sizes;

            render(&mut app);
            app.should_render = false;
        }

        app.keyboard_handler.clone()(&mut app);
    }

    clear_screen();
}

fn print_help() {
    let keybinds = [
        ("h", "Toggle Help"),
        ("m", "Toggle help Message"),
        ("r", "Force Refresh the series"),
        ("\\d+(j|k)", "Move down or up by this many"),
        ("G", "Move to last series"),
        ("gg", "Move to first series"),
    ];

    for (key, desc) in keybinds.iter() {
        println!(
            "{}{: >4}{:?}: {}",
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
        println!();
        return;
    }

    app.renderer.clone()(app);
    println!();

    match &app.toast {
        Some(msg) => {
            println!("Note: {}", msg);
            app.toast = None;
        }
        None => (),
    }

    if app.should_show_help_msg {
        println!("Press [h] for keybinds");
    }
}

fn main_render(app: &mut App) {
    if app.should_show_help {
        print_help();
        println!();
        return;
    }

    let data = &mut app.data;

    if data.available_series.is_empty() || data.ignore_cached_series {
        let series = Series::get(data.take, data.skip).unwrap_or_else(|_| vec![]);
        data.available_series.clone_from(&series);

        data.ignore_cached_series = false;
    }

    print_series_table(&data.available_series, data.hovered_series_idx);
}

fn main_key_handler(app: &mut App) {
    app.should_render = true;

    let user_input = input::get();
    let input = parse_input(user_input.trim());

    let binding = input.actions.iter().collect::<String>();
    let binding = binding.as_str();

    match binding {
        "q" => app.should_exit = true,

        "h" => app.should_show_help = !app.should_show_help,
        "m" => app.should_show_help_msg = !app.should_show_help_msg,

        "r" => app.data.ignore_cached_series = true,

        "i" => start_inserting(app),

        "k" => move_up(app, &input),
        "j" => move_down(app, &input),

        "G" => move_max(app),
        "gg" => move_min(app),

        "-" => scroll_up(app, &input),
        "+" => scroll_down(app, &input),

        "" => {
            if input.digits_prefix.is_some() {
                move_to(app, &input);
            }
        }

        _ => {}
    }
}
