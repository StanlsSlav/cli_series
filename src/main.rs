mod db;
mod series;
mod term;
mod printer;

use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    event::{poll, read, KeyEventKind},
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute},
};
use printer::print_series_table;
use series::Series;
use term::clear_screen;

struct App {
    should_render: bool,
    should_exit: bool,
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
        data: Data {
            hovered_serie_idx: None,
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

fn render(app: &mut App) -> Result<(), Box<dyn Error>> {
    clear_screen();

    let data = &mut app.data;

    if data.available_series.is_empty() || data.ignore_cached_series {
        let series = Series::get(data.take, data.skip)?;
        data.available_series.clone_from(&series);

        data.ignore_cached_series = false;
    }

    print_series_table(&data.available_series, data.hovered_serie_idx)?;
    Ok(())
}

fn handle_input(app: &mut App) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match read()? {
            crossterm::event::Event::FocusGained
            | crossterm::event::Event::FocusLost
            | crossterm::event::Event::Mouse(_)
            | crossterm::event::Event::Paste(_) => (),
            crossterm::event::Event::Resize(_, _) => app.should_render = true,
            crossterm::event::Event::Key(key_ev) => handle_keyboard_input(app, key_ev)?,
        }
    }

    Ok(())
}

fn handle_keyboard_input(
    app: &mut App,
    event: crossterm::event::KeyEvent,
) -> Result<(), Box<dyn Error>> {
    if event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let crossterm::event::KeyCode::Char(chr) = event.code {
        app.should_render = true;

        match chr {
            'q' => {
                app.should_exit = true;
            }
            'r' | 'u' => app.data.ignore_cached_series = true,
            'k' => {
                if let Some(idx) = app.data.hovered_serie_idx {
                    app.data.hovered_serie_idx = Some(idx.saturating_sub(1).min(0));
                }
            }
            'j' => {
                let max_series = app.data.available_series.len();

                if let Some(idx) = app.data.hovered_serie_idx {
                    app.data.hovered_serie_idx = Some(idx.saturating_add(1).max(max_series));
                }
            }
            _ => app.should_render = false,
        }
    }

    Ok(())
}
