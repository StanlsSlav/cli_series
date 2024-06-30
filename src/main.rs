mod db;
mod printer;
mod series;
mod term;

use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use printer::print_series_table;
use series::Series;
use term::clear_screen;

struct App {
    should_render: bool,
    should_exit: bool,
    should_show_help: bool,
    should_show_help_msg: bool,
    data: Data,
}

struct Data {
    hovered_serie_idx: Option<usize>,
    available_series: Vec<Series>,
    ignore_cached_series: bool,
    take: Option<usize>,
    skip: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App {
        should_render: true,
        should_exit: false,
        should_show_help: false,
        should_show_help_msg: true,
        data: Data {
            hovered_serie_idx: Some(0),
            available_series: Vec::new(),
            ignore_cached_series: false,
            take: Some(24),
            skip: None,
        },
    };

    while !app.should_exit {
        if app.should_render {
            render(&mut app)?;
            app.should_render = false;
        }

        handle_input(&mut app)?;
    }

    clear_screen();
    execute!(
        stdout(),
        ResetColor,
        SetAttribute(Attribute::SlowBlink),
        Print("Bye!")
    )?;
    Ok(())
}

fn print_help() -> Result<(), Box<dyn Error>> {
    let keybinds = [
        ('h', "Toggle [h]elp"),
        ('m', "Toggle help [m]essage"),
        ('r', "Force [r]efresh the series"),
        ('k', "Move up"),
        ('j', "Move down"),
        ('G', "Move to last serie"),
    ];

    for (key, desc) in keybinds.iter() {
        execute!(
            stdout(),
            SetForegroundColor(Color::Magenta),
            Print(key),
            ResetColor,
            Print(format!(": {}\n", desc)),
        )?;
    }

    Ok(())
}

fn render(app: &mut App) -> Result<(), Box<dyn Error>> {
    clear_screen();

    if app.should_show_help {
        print_help()?;
        return Ok(());
    }

    let data = &mut app.data;

    if data.available_series.is_empty() || data.ignore_cached_series {
        let series = Series::get(data.take, data.skip)?;
        data.available_series.clone_from(&series);

        data.ignore_cached_series = false;
    }

    print_series_table(&data.available_series, data.hovered_serie_idx)?;

    if app.should_show_help_msg {
        println!("\nPress [h] for keybinds");
    }

    Ok(())
}

fn handle_input(app: &mut App) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match read()? {
            Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => (),
            Event::Resize(_, _) => app.should_render = true,
            Event::Key(key_ev) => handle_keyboard_input(app, key_ev)?,
        }
    }

    Ok(())
}

fn handle_keyboard_input(app: &mut App, event: KeyEvent) -> Result<(), Box<dyn Error>> {
    if event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let KeyCode::Char(chr) = event.code {
        app.should_render = true;

        match chr {
            'q' => {
                app.should_exit = true;
            }

            'h' => {
                app.should_show_help = !app.should_show_help;
            }

            'm' => app.should_show_help_msg = !app.should_show_help_msg,

            'r' => app.data.ignore_cached_series = true,

            'k' => {
                if let Some(idx) = app.data.hovered_serie_idx {
                    app.data.hovered_serie_idx = Some(idx.saturating_sub(1).max(0));
                }
            }

            'j' => {
                let max_series = app.data.available_series.len() - 1;

                if let Some(idx) = app.data.hovered_serie_idx {
                    app.data.hovered_serie_idx = Some(idx.saturating_add(1).min(max_series));
                }
            }

            'G' => {
                app.data.hovered_serie_idx = Some(app.data.available_series.len() - 1);
            }

            _ => app.should_render = false,
        }
    }

    Ok(())
}
