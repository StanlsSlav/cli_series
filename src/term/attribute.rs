use super::CSI;

pub(crate) fn invert() -> String {
    format!("{CSI}7m")
}

pub(crate) fn reset() -> String {
    format!("{CSI}0m")
}
