use std::error::Error;

pub(crate) fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub(crate) fn get_size() -> Result<(u16, u16), Box<dyn Error>> {
    let (width, height) = crossterm::terminal::size()?;
    Ok((width, height))
}
