use crate::engine::Object;
use crate::game::GameController;
use crate::geometry::ScreenCoord;
use crate::ui::Drag;
use macroquad::camera::Camera3D;
use macroquad::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseAction {
    None,
    Dragging,
    Drop,
}

pub enum KbdAction {
    Quit,
    ReloadShader,
    StartGame,
    Reset,
}

#[derive(Debug, Clone)]
pub struct ControlStatus {
    /// Current action; e.g. `Drag`, `Drop`.
    pub action: MouseAction,
    /// Mouse position in pixel coords.
    pub mouse_pos: Option<ScreenCoord>,
    /// Object under the cursor.
    pub hovering: Option<Object>,
    /// Representation of an ongoing drag action.
    pub dragging: Option<Drag>,
    /// Tile available for the current ongoing action.
    pub targeting: Option<Object>,
}

impl Default for ControlStatus {
    fn default() -> ControlStatus {
        ControlStatus {
            action: MouseAction::None,
            mouse_pos: None,
            hovering: None,
            dragging: None,
            targeting: None,
        }
    }
}

impl ControlStatus {
    pub fn update(&mut self, game: &GameController, camera: &Camera3D) {
        self.mouse_pos = ControlStatus::get_mouse_position(camera);
        self.hovering = self.get_hovered_object(game);
        self.targeting = self.get_targeted_tile(game);
        self.action = self.update_mouse_action();
    }

    fn update_mouse_action(&self) -> MouseAction {
        if is_mouse_button_released(MouseButton::Left) {
            if self.action == MouseAction::Dragging {
                return MouseAction::Drop;
            }
        } else if is_mouse_button_pressed(MouseButton::Left) && self.hovering.is_some() {
            return MouseAction::Dragging;
        } else if is_mouse_button_down(MouseButton::Left) {
            return self.action.clone();
        }
        MouseAction::None
    }

    fn get_mouse_position(camera: &Camera3D) -> Option<ScreenCoord> {
        let (m_pos_x, m_pos_y) = mouse_position();
        let mouse_vec_ndc =
            2.0 * vec3(m_pos_x / screen_width(), m_pos_y / screen_height(), 0.5) - 1.0;
        let mtx_inv = camera.matrix().inverse();
        let near_point = mtx_inv.project_point3(mouse_vec_ndc.with_z(-1.0));
        let far_point = mtx_inv.project_point3(mouse_vec_ndc.with_z(1.0));

        let ray_origin = camera.position;
        let ray_dir = (far_point - near_point).normalize();
        if ray_dir.z.abs() > f32::EPSILON {
            let t = -ray_origin.z / ray_dir.z;
            let xy_intersection = ray_origin + t * ray_dir;
            return Some(ScreenCoord::new(xy_intersection.x, xy_intersection.y));
        }
        None
    }

    fn get_targeted_tile(&self, game: &GameController) -> Option<Object> {
        match &self.hovering {
            None => self.get_hovered_tile(game),
            Some(object) => {
                let targetable_player = if self.action == MouseAction::None {
                    game.current_player()
                } else {
                    game.current_player().opponent()
                };
                if let Some(m_pos) = self.mouse_pos {
                    if object.owned_by(&targetable_player) {
                        return game.get_tile_at_pos(&m_pos);
                    }
                }
                None
            }
        }
    }

    fn get_hovered_object(&self, game: &GameController) -> Option<Object> {
        match self.mouse_pos {
            Some(coord) => game.get_object_at_pos(&coord),
            None => None,
        }
    }

    fn get_hovered_tile(&self, game: &GameController) -> Option<Object> {
        match self.mouse_pos {
            Some(coord) => {
                if let Some(tile) = game.get_tile_at_pos(&coord) {
                    if let Some(obj) = game.board.contents(&tile.coord) {
                        if !obj.owned_by(&game.current_player()) {
                            return None;
                        }
                    }
                    Some(tile)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
