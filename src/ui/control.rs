use crate::engine::Object;
use crate::geometry::ScreenCoord;
use crate::ui::Drag;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseAction {
    None,
    Dragging,
    Drop,
}

pub enum KbdAction {
    Quit,
    ReloadShader,
}

#[derive(Debug, Clone)]
pub struct ControlStatus {
    pub mouse_pos: ScreenCoord,
    pub action: MouseAction,
    pub hovering: Option<Object>,
    pub dragging: Option<Drag>,
    pub targeting: Option<Object>,
}
