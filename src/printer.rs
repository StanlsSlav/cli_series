use crate::series::Series;
use crate::term;
use crate::term::{attribute, color::Color};

pub(crate) fn print_series_table(printable_series: &[Series], selected_idx: usize) {
    print_header();
    print_separator("-", true);

    let series = printable_series.iter();
    for (i, series) in series.enumerate() {
        let is_selected = i == selected_idx;

        print_row(series, i as i32, is_selected);
        println!();
    }
}

fn print_header() {
    println!("Id | Guid | Name | Finished | Airing Finished | Total Episodes | Current Episode");
}

fn print_separator(sep: &str, do_repeat: bool) {
    let (width, _) = match term::get_size() {
        Some(sizes) => (sizes.cols, sizes.rows),
        None => (0, 0),
    };

    let sep = if do_repeat {
        &sep.repeat(width as usize - sep.len() + 1).to_string()
    } else {
        sep
    };

    print!("{}", sep);
}

fn print_row(series: &Series, idx: i32, is_selected: bool) {
    let invert = attribute::invert();
    let attribute = match is_selected {
        true => Some(invert.as_str()),
        false => None,
    };

    print_cell(idx + 1, None, None);
    print_separator(" | ", false);

    print_cell(
        series.guid[..4].to_owned() + "...",
        Some(Color::Rgb(190, 190, 190)),
        attribute,
    );
    print_separator(" | ", false);

    print_cell(&series.name, Some(Color::Green), attribute);
    print_separator(" | ", false);

    print_cell(series.is_finished, Some(Color::Red), attribute);
    print_separator(" | ", false);

    print_cell(series.is_airing_finished, Some(Color::Blue), attribute);
    print_separator(" | ", false);

    print_cell(series.total_episodes, Some(Color::Green), attribute);
    print_separator(" | ", false);

    print_cell(series.current_episode, Some(Color::Green), attribute);
}

fn print_cell<T: ToString>(cell: T, color: Option<Color>, style: Option<&str>) {
    if let Some(color) = color {
        print!("{}", color);
    }

    if let Some(style) = style {
        print!("{}", style);
    };

    print!("{}{}", cell.to_string(), attribute::reset());
}
