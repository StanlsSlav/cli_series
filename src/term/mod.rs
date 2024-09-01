use std::io::IsTerminal;

use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};

pub(crate) mod attribute;
pub(crate) mod color;

pub(crate) const CSI: &str = "\x1B[";

pub(crate) fn clear_screen() {
    print!("{CSI}2J{CSI}1;1H");
}

#[derive(Debug)]
pub(crate) struct Size {
    pub rows: u16,
    pub cols: u16,
}

#[repr(C)]
#[derive(Debug)]
struct UnixSize {
    rows: c_ushort,
    cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

// https://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
pub(crate) fn get_size() -> Option<Size> {
    if !std::io::stdout().is_terminal() {
        return None;
    }

    let mut us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };

    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut us) };

    if r == 0 {
        Some(Size {
            rows: us.rows,
            cols: us.cols,
        })
    } else {
        None
    }
}
