use macroquad::prelude::*;
use super::HexCoord;


#[derive(Copy, Clone, Debug)]
pub struct ScreenCoord {
    pub x: f32,
    pub y: f32,
    pub screen_size: f32,
}


impl ScreenCoord {
    pub fn new(x: f32, y:f32, board_size: usize) -> ScreenCoord {
        ScreenCoord {
            x,
            y,
            screen_size: ScreenCoord::screen_size(board_size),
        }
    }

    pub fn from_hexcoord(coord: &HexCoord) -> ScreenCoord {
        let screen_size = ScreenCoord::screen_size(coord.board_size);
        let offset_x = screen_size * (1 + coord.y) as f32;
        let offset_y = 2.0 * screen_size;
        let x = offset_x + (2.15 * screen_size * coord.x as f32);
        let y = offset_y + (1.85 * screen_size * coord.y as f32);
        ScreenCoord {x, y, screen_size}
    }


    pub fn mouse_pos(board_size: usize) -> ScreenCoord {
        let (x, y) = mouse_position();
        ScreenCoord::new(x, y, board_size)
    }

    pub fn is_close(&self, other: ScreenCoord) -> bool {
        let delta = self.sub(&other);
        let distance_sq = delta.x.powi(2) + delta.y.powi(2);
        distance_sq < 0.75 * self.screen_size.powi(2)
    }

    pub fn add(&self, other: &ScreenCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.x + other.x,
            y: self.y + other.y,
            screen_size: self.screen_size,
        }
    }

    pub fn add_v(&self, vec: Vec2) -> ScreenCoord {
        ScreenCoord {
            x: self.x + vec.x,
            y: self.y + vec.y,
            screen_size: self.screen_size,
        }
    }

    pub fn sub(&self, other: &ScreenCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.x - other.x,
            y: self.y - other.y,
            screen_size: self.screen_size,
        }
    }

    pub fn scale(&self, factor: f32) -> ScreenCoord {
        ScreenCoord {
            x: self.x * factor,
            y: self.y * factor,
            screen_size: self.screen_size * factor,
        }
    }

    pub fn screen_size(board_size: usize) -> f32 {
        f32::min(
            0.33 * screen_width() / board_size as f32,
            0.5 * screen_height() / (1 + board_size) as f32,
        )
    }
}
