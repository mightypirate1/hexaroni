use macroquad::prelude::*;
use crate::{game::{statuses::Status, Object, ObjectType}, geometry::ScreenCoord};


pub fn render_dragged(object: &Object, pos: ScreenCoord, _time: f32) {
    let color = match object.otype {
        ObjectType::Jumper => Color::from_rgba(255, 94, 7, 255),
        ObjectType::Dasher => Color::from_rgba(157, 24, 250, 255),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, 255),
        ObjectType::Tile => panic!("Invalid object type"),
    };
    draw_bordered_circle(pos, &color, 0.5);
}

pub fn render_tile(tile: &Object, _time: f32) {
    draw_bordered_hexagon(
        &tile.pos,
        tile.pos.screen_size,
        &Color::from_hex(0x333333),
        1.0,
    );
}

pub fn render_non_tile_object(object: &Object, time: f32) {
    let color = object_color(&object.otype);
    let alpha = if object.statuses.contains(&Status::Dragged) { 0.5 } else { 1.0 };
    draw_bordered_circle(object.get_screen_coord(time), &color, alpha);
}

pub fn render_background(_time: f32) {
    clear_background(Color::new(0.12, 0.075, 0.11, 1.0));
}

pub fn draw_bordered_hexagon(pos: &ScreenCoord, size: f32, color: &Color, alpha: f32) {
    draw_hexagon(
        pos.x,
        pos.y,
        size,
        0.15 * size,
        true,
        with_alpha(&border_color(color), alpha),
        with_alpha(color, alpha),
    );
}

fn draw_bordered_circle(pos: ScreenCoord, color: &Color, alpha: f32) {
    draw_circle(
        pos.x,
        pos.y,
        0.80 * pos.screen_size,
        with_alpha(&border_color(color), alpha),
    );
    draw_circle(
        pos.x,
        pos.y,
        0.65 * pos.screen_size,
        with_alpha(color, alpha),
    );
}

fn border_color(color: &Color) -> Color {
    let blend = 0.05;
    Color::from_vec(blend * color.to_vec() + (1.0 - blend) * BLACK.to_vec())
}

fn with_alpha(color: &Color, alpha: f32) -> Color {
    Color::from_vec(color.to_vec().with_w(alpha))
}

fn object_color(otype: &ObjectType) -> Color {
    match otype {
        ObjectType::Jumper => Color::from_rgba(255, 94, 7, 255),
        ObjectType::Dasher => Color::from_rgba(157, 24, 250, 255),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, 255),
        ObjectType::Tile => panic!("Invalid object type"),
    }
}
