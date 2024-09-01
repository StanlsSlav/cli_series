use crate::{series::Series, term::Size, user::create_input::CreateInput, Mode};
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

    take_backup: usize,
    pub(crate) take: usize,
    pub(crate) skip: usize,
    pub(crate) total_series: usize,
}

impl Data {
    pub(crate) fn default(total_series: usize) -> Self {
        Self {
            hovered_series_idx: 0,
            available_series: vec![],
            ignore_cached_series: true,
            take_backup: 16,
            take: 16,
            skip: 0,
            total_series,
        }
    }

    pub(crate) fn set_take(&mut self, take: usize) {
        self.take_backup = self.take;
        self.take = take;
    }

    pub(crate) fn restore_take(&mut self) {
        self.take = self.take_backup;
    }
}
