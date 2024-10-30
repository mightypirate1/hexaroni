use macroquad::prelude::*;
use crate::{game::{statuses::Status, Object, ObjectType, Player}, geometry::ScreenCoord};
use super::Drag;


pub fn render_dragged_object(drag: &Drag, pos: ScreenCoord, _time: f32) {
    let color = match drag.object.otype {
        ObjectType::Jumper => Color::from_rgba(255, 94, 7, 255),
        ObjectType::Dasher => Color::from_rgba(157, 24, 250, 255),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, 255),
        ObjectType::Tile => panic!("Invalid object type"),
    };
    let border_color = border_color(&color, &drag.object);
    draw_bordered_circle(pos, &color, &border_color, 0.5);
}


pub fn render_tile(tile: &Object, time: f32) {
    let pos = tile.get_display_pos(time);
    draw_bordered_hexagon(
        &pos,
        pos.screen_size,
        &Color::from_hex(0x333333),
        1.0,
    );
}


pub fn render_non_tile_object(object: &Object, time: f32) {
    let color = object_color(&object.otype);
    let border_color = border_color(&color, object);
    let alpha = if object.statuses.contains(&Status::Dragged) { 0.5 } else { 1.0 };
    draw_bordered_circle(object.get_display_pos(time), &color, &border_color, alpha);
}


pub fn render_background(_time: f32) {
    clear_background(Color::new(0.12, 0.075, 0.11, 1.0));
}


pub fn render_tile_status_color(tile: &Object, color: &Color, time: f32) {
    let pos = tile.get_display_pos(time);
    draw_bordered_hexagon(&pos, pos.screen_size, color, 0.35);
}


fn draw_bordered_hexagon(pos: &ScreenCoord, size: f32, color: &Color, alpha: f32) {
    draw_hexagon(
        pos.x,
        pos.y,
        size,
        0.15 * size,
        true,
        with_alpha(&mix_color(color, &BLACK, 0.05), alpha),
        with_alpha(color, alpha),
    );
}


fn object_color(otype: &ObjectType) -> Color {
    match otype {
        ObjectType::Jumper => Color::from_rgba(255, 94, 7, 255),
        ObjectType::Dasher => Color::from_rgba(157, 24, 250, 255),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, 255),
        ObjectType::Tile => panic!("Invalid object type"),
    }
}


fn draw_bordered_circle(pos: ScreenCoord, color: &Color, border_color: &Color, alpha: f32) {
    draw_circle(
        pos.x,
        pos.y,
        0.80 * pos.screen_size,
        with_alpha(border_color, alpha),
    );
    draw_circle(
        pos.x,
        pos.y,
        0.65 * pos.screen_size,
        with_alpha(color, alpha),
    );
}


fn border_color(color: &Color, object: &Object) -> Color {
    let blend = 0.5;
    let mixin_color = match &object.player {
        Player::A => BLACK,
        Player::B => WHITE,
        _ => BLACK,
    };
    mix_color(&color, &mixin_color, blend)
}


fn mix_color(x: &Color, y: &Color, alpha: f32) -> Color {
    Color::from_vec(alpha * x.to_vec() + (1.0 - alpha) * y.to_vec())
}


fn with_alpha(color: &Color, alpha: f32) -> Color {
    Color::from_vec(color.to_vec().with_w(alpha))
}
