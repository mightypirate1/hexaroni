use super::HexCoord;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScreenCoord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ScreenCoord {
    pub fn new(x: f32, y: f32) -> ScreenCoord {
        ScreenCoord { x, y, z: 0.0 }
    }

    pub fn from_hexcoord(coord: &HexCoord) -> ScreenCoord {
        let s = coord.board_size as f32;
        let cx = coord.x as f32 - 0.5 * s;
        let cy = coord.y as f32 - 0.5 * s;
        let offset_x = 1.2 + cy;
        let offset_y = 1.2;
        let x = offset_x + (2.15 * cx);
        let y = offset_y + (1.85 * cy);

        let x = x - 0.6;
        let y = y - 0.6;

        ScreenCoord { x, y, z: 0.0 }
    }

    pub fn dist_from(&self, other: &ScreenCoord) -> f32 {
        (self.as_vec() - other.as_vec()).length()
    }

    pub fn as_vec(&self) -> Vec3 {
        vec3(self.x, self.y, self.z)
    }

    pub fn with_x(&self, x: f32) -> ScreenCoord {
        ScreenCoord {
            x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn with_y(&self, y: f32) -> ScreenCoord {
        ScreenCoord {
            x: self.x,
            y,
            z: self.z,
        }
    }
    pub fn with_z(&self, z: f32) -> ScreenCoord {
        ScreenCoord {
            x: self.x,
            y: self.y,
            z,
        }
    }

    pub fn add(&self, other: &ScreenCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &ScreenCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn scale(&self, factor: f32) -> ScreenCoord {
        ScreenCoord {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}
