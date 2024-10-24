use macroquad::prelude::*;
use crate::game::HexCoord;


#[derive(Copy, Clone, Debug)]
pub struct ScreenCoord {
    pub x: f32,
    pub y: f32,
    pub screen_size: f32,
}


impl ScreenCoord {
    pub fn from_hexcoord(coord: &HexCoord) -> ScreenCoord {
        let screen_size = ScreenCoord::screen_size(coord.board_size);
        let offset_x = screen_size * (1 + coord.y) as f32;
        let offset_y = 2.0 * screen_size;
        let x = offset_x + (2.0 * screen_size * coord.x as f32);
        let y = offset_y + (2.0 * screen_size * coord.y as f32);
        ScreenCoord {x, y, screen_size}
    }

    pub fn add(&self, other: &ScreenCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.x + other.x,
            y: self.y + other.y,
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
