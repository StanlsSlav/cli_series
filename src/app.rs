use termsize::Size;

use crate::{series::Series, user::create_input::CreateInput, Mode};
use std::sync::Arc;

pub(crate) struct App {
    pub(crate) should_render: bool,
    pub(crate) should_exit: bool,
    pub(crate) should_show_help: bool,
    pub(crate) should_show_help_msg: bool,

    #[allow(clippy::type_complexity)]
    pub(crate) keyboard_handler: Arc<dyn Fn(&mut App)>,
    pub(crate) renderer: Arc<dyn Fn(&mut App)>,

    pub(crate) data: Data,
    pub(crate) create_data: Vec<CreateInput>,

    pub(crate) term_size: Size,
    pub(crate) mode: Mode,

    pub(crate) toast: Option<String>,
}

pub(crate) struct Data {
    pub(crate) hovered_series_idx: usize,
    pub(crate) available_series: Vec<Series>,
    pub(crate) ignore_cached_series: bool,

    pub(crate) take: usize,
    pub(crate) skip: usize,
    pub(crate) total_series: usize,
}
