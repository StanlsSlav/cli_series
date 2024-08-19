use termsize::Size;

use crate::{Mode, series::Series};
use std::sync::Arc;

pub(crate) struct App {
    pub(crate) should_render: bool,
    pub(crate) should_exit: bool,
    pub(crate) should_show_help: bool,
    pub(crate) should_show_help_msg: bool,

    #[allow(clippy::type_complexity)]
    pub(crate) keyboard_handler: Arc<dyn Fn(&mut App)>,

    pub(crate) data: Data,
    pub(crate) term_size: Size,
    pub(crate) mode: Mode,
}

pub(crate) struct Data {
    pub(crate) hovered_series_idx: Option<usize>,
    pub(crate) available_series: Vec<Series>,
    pub(crate) ignore_cached_series: bool,

    pub(crate) take: Option<usize>,
    pub(crate) skip: Option<usize>,
}
