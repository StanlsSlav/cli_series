use std::{error::Error, io::stdout};

use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    ExecutableCommand,
};

use crate::{series::Series, term::get_size};

struct Paddings {
    name_cell: usize,
}

pub(crate) fn print_series_table(
    series: &Vec<Series>,
    selected_idx: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    print_header()?;
    print_separator(None, None)?;

    let series = series.iter();
    for (i, serie) in series.enumerate() {
        print_row(serie, i == selected_idx.unwrap_or(0))?;
        println!();
    }

    Ok(())
}

fn print_header() -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        Print("Guid | Name | Finished | Airing Finished | Total Episodes | Current Episode\n"),
    )?;

    Ok(())
}

fn print_separator(sep: Option<&str>, do_repeat: Option<bool>) -> Result<(), Box<dyn Error>> {
    let (width, _) = get_size()?;
    let sep = sep.unwrap_or("-");
    let do_repeat = do_repeat.unwrap_or(true);

    let sep = if do_repeat {
        &sep.repeat(width as usize - sep.len() + 1).to_string()
    } else {
        sep
    };

    print!("{}", sep);

    Ok(())
}

fn print_row(serie: &Series, is_selected: bool) -> Result<(), Box<dyn Error>> {
    let attribute = match is_selected {
        true => Some(Attribute::Reverse),
        false => None,
    };

    print_cell(
        serie.guid[..4].to_owned() + "...",
        Some(Color::Grey),
        attribute,
    )?;
    print_separator(Some(" | "), Some(false))?;

    print_cell(&serie.name, Some(Color::Green), attribute)?;
    print_separator(Some(" | "), Some(false))?;

    print_cell(serie.is_finished, Some(Color::Red), attribute)?;
    print_separator(Some(" | "), Some(false))?;

    print_cell(serie.is_airing_finished, Some(Color::Blue), attribute)?;
    print_separator(Some(" | "), Some(false))?;

    print_cell(serie.total_episodes, Some(Color::Green), attribute)?;
    print_separator(Some(" | "), Some(false))?;

    print_cell(serie.current_episode, Some(Color::Green), attribute)?;

    Ok(())
}

fn print_cell<T: ToString>(
    cell: T,
    color: Option<Color>,
    attribute: Option<Attribute>,
) -> Result<(), Box<dyn Error>> {
    let mut out = stdout();
    out.execute(SetForegroundColor(color.unwrap_or(Color::White)))?;

    if let Some(attr) = attribute {
        out.execute(SetAttribute(attr))?;
    };

    out.execute(Print(cell.to_string()))?;
    out.execute(ResetColor)?;

    Ok(())
}
