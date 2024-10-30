#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Selected,
    Dragged,
    Hovered,
    Targeted,
    Targetable,
}
