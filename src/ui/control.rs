use crate::geometry::ScreenCoord;
use crate::game::Object;


#[derive(Debug, Clone)]
pub enum MouseAction {
    None,
    Dragging,
    Drop,
}

pub enum KbdAction {
    Quit,
}

#[derive(Debug, Clone)]
pub struct ControlStatus {
    pub mouse_pos: ScreenCoord,
    pub action: MouseAction,
    pub hovering: Option<Object>,
    pub dragging: Option<Object>,
    pub targeting: Option<Object>,
}
