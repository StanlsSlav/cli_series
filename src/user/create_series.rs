use crate::{
    app::App,
    input,
    keybinds::{
        nav::{move_down, move_max, move_min, move_to, move_up, start_listing},
        parse_input,
    },
    series::Series,
    term::attribute,
    Mode,
};

pub(crate) fn create_render(app: &mut App) {
    for (i, input) in app.create_data.iter().enumerate() {
        let label = input.label.clone();

        if i == app.data.hovered_series_idx {
            print!("{}", attribute::invert());
        }

        if app.mode == Mode::Edit && i != app.data.hovered_series_idx {
            continue;
        }

        println!("{}{}: {:?}", label, attribute::reset(), input.raw_value);
    }
}

pub(crate) fn create_key_handler(app: &mut App) {
    let user_input = input::get();

    if app.mode == Mode::Edit {
        app.create_data[app.data.hovered_series_idx].raw_value = user_input;
        app.mode = Mode::Navigation;
        return;
    }

    let input = parse_input(user_input.trim());
    let binding = input.actions.iter().collect::<String>();
    let binding = binding.as_str();

    match binding {
        "q" => start_listing(app),

        "k" => move_up(app, &input),
        "j" => move_down(app, &input),

        "G" => move_max(app),
        "gg" => move_min(app),

        "" => {
            if input.digits_prefix.is_some() {
                move_to(app, &input);
            }
        }

        "e" => app.mode = Mode::Edit,
        "i" => {
            let form = &app.create_data;
            let mut series = Series::new(
                form[0].raw_value.clone(),
                form[1].get_bool(),
                form[2].get_bool(),
                form[3].get_i32(),
                form[4].get_i32(),
            );

            match series.try_insert() {
                Ok(_) => app.toast = Some("Series created!".to_string()),
                Err(er) => app.toast = Some(er.to_string()),
            }
        }
        _ => {}
    }
}
