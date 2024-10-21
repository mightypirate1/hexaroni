use macroquad::prelude::*;
use crate::game::{Tile, Object, ObjectType};
use super::ScreenCoord;

pub fn tile(tile: &Tile) {
    let screen_coord = ScreenCoord::from_hexcoord(&tile.coord);
    draw_hexagon(
        screen_coord.x,
        screen_coord.y,
        screen_coord.screen_size,
        0.12 * screen_coord.screen_size,
        true,
        Color::from_hex(0x000000),
        Color::from_hex(0x444444),
    );
}

pub fn object(object: &Object) {
    let color = match object.otype {
        ObjectType::Jumper => Color::from_hex(0xb04311),
        ObjectType::Dasher => Color::from_hex(0x6122c7),
        ObjectType::Wall => Color::from_hex(0x111111),
    };
    let screen_coord = ScreenCoord::from_hexcoord(&object.coord);
    draw_circle(screen_coord.x, screen_coord.y, 0.75 * screen_coord.screen_size, color);
}

pub fn background() {
    clear_background(Color::new(0.2, 0.15, 0.22, 1.0));
}