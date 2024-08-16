pub(crate) mod attribute;
pub(crate) mod color;

pub(crate) const CSI: &str = "\x1B[";

pub(crate) fn clear_screen() {
    print!("{CSI}2J{CSI}1;1H");
}
