use macroquad::prelude::*;
use crate::game::{statuses::Status, Object, ObjectType, Game};

pub fn render_game(game: &Game, time: f32) {
    render_background(time);
    for object in &game.board.objects {
        match object.otype {
            ObjectType::Tile => {
                render_tile(object, time);
                for status in &object.statuses {
                    render_status(status, object, time);
                }
            },
            _ => {
                render_non_tile_object(object, time);
                for status in &object.statuses {
                    render_status(status, object, time);
                }
            }
        }
    }
}

fn render_tile(tile: &Object, _time: f32) {
    draw_hexagon(
        tile.pos.x,
        tile.pos.y,
        tile.pos.screen_size,
        0.15 * tile.pos.screen_size,
        true,
        Color::from_hex(0x000000),
        Color::from_hex(0x333333),
    );
}

fn render_non_tile_object(object: &Object, time: f32) {
    let screen_coord = object.get_screen_coord(time);
    let color = match object.otype {
        ObjectType::Jumper => Color::from_rgba(255, 94, 7, 255),
        ObjectType::Dasher => Color::from_rgba(157, 24, 250, 255),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, 255),
        ObjectType::Tile => panic!("Invalid object type"),
    };
    let blend = 0.05;
    let border_color = Color::from_vec(
        blend * color.to_vec() + (1.0 - blend) * BLACK.to_vec(),
    );
    draw_circle(
        screen_coord.x,
        screen_coord.y,
        0.80 * screen_coord.screen_size,
        border_color,
    );
    draw_circle(
        screen_coord.x,
        screen_coord.y,
        0.65 * screen_coord.screen_size,
        color,
    );
}

fn render_background(_time: f32) {
    clear_background(Color::new(0.12, 0.075, 0.11, 1.0));
}

fn render_status(status: &Status, object: &Object, time: f32) {
    let screen_coord = object.get_screen_coord(time);
    let color = match status {
        Status::Selected => Color::new(0.1, 0.87, 0.05, 0.2),
        Status::Hovered => Color::new(0.1, 0.05, 0.87, 0.2),
        Status::Targeted => Color::new(0.87, 0.1, 0.05, 0.2),
    };
    draw_circle(screen_coord.x, screen_coord.y, 0.1 * screen_coord.screen_size, color);
}
