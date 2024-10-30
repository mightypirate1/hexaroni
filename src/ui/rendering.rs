use macroquad::prelude::*;
use crate::game::Game;
use crate::geometry::ScreenCoord;
use super::{draw, control::ControlStatus};


pub fn render(game: &Game, time: f32, control_status: &ControlStatus, render_target: &RenderTarget) {
    let w = screen_width();
    let h = screen_height();
    set_camera(&Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        target: vec2(0.5 * w, 0.5 * h),
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    // background
    draw::render_background(time);
    
    // animated tiles (first since they are presumed to fall or something...)
    for tile in game.board.tiles.iter().filter(|&t| t.animation.is_some()) {
        draw::render_tile(tile, time);
    }
    // non-animated tiles
    for tile in game.board.tiles.iter().filter(|&t| t.animation.is_none()) {
        draw::render_tile(tile, time);
        if let Some(targeting) = &control_status.targeting {
            draw::render_tile_status_color(targeting, &PINK, time);
        }
        if let Some(drag) = &control_status.dragging {
            if drag.targets.contains(&tile.coord) {
                draw::render_tile_status_color(&tile, &SKYBLUE, time);
            }
        }
    }
    for object in game.board.objects.iter().filter(|&t| t.animation.is_none()) {
        draw::render_non_tile_object(object, time);
    }
    for object in game.board.objects.iter().filter(|&t| t.animation.is_some()) {
        draw::render_non_tile_object(object, time);
    }
    if let Some(drag) = &control_status.dragging {
        let screen_coord = ScreenCoord::mouse_pos(game.board.size);
        draw::render_dragged_object(drag, screen_coord, time);
    }

    set_default_camera();
    draw_texture_ex(
        &render_target.texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
}
