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
        0.12 * tile.pos.screen_size,
        true,
        Color::from_hex(0x000000),
        Color::from_hex(0x444444),
    );
}

fn render_non_tile_object(object: &Object, time: f32) {
    for status in &object.statuses {
        render_status(status, object, time);
    }
    let screen_coord = object.get_screen_coord(time);
    let alpha = 255 - 25;
    // let color = match object.otype {
    //     ObjectType::Jumper => Color::from_rgba(69, 6, 86, alpha),
    //     ObjectType::Dasher => Color::from_rgba(107, 28, 8, alpha),
    //     ObjectType::Wall => Color::from_rgba(5, 5, 5, alpha),
    //     ObjectType::Tile => panic!("Invalid object type"),
    // };
    // let mut border_color = Color::from_vec(1.5 * color.to_vec());
    let color = match object.otype {
        ObjectType::Jumper => Color::from_rgba(11 * 16, 35, 17, alpha),
        ObjectType::Dasher => Color::from_rgba(97, 34, 162, alpha),
        ObjectType::Wall => Color::from_rgba(19, 19, 19, alpha),
        ObjectType::Tile => panic!("Invalid object type"),
    };
    let mut border_color = Color::from_vec(0.4 * color.to_vec());
    border_color.a = 0.92;  //alpha as f32 / 255.0;
    draw_circle(
        screen_coord.x,
        screen_coord.y,
        0.85 * screen_coord.screen_size,
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
    clear_background(Color::new(0.2, 0.15, 0.22, 1.0));
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
