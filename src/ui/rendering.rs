use macroquad::prelude::*;
use crate::game::{Game, Object, ObjectType};
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

    let tiles: Vec<&Object> = game.board.objects.iter()
        .filter(|&o| o.otype == ObjectType::Tile).collect();
    let non_tiles: Vec<&Object> = game.board.objects.iter()
        .filter(|&o| o.otype != ObjectType::Tile).collect();

    draw::render_background(time);
    for tile in tiles.iter().filter(|&t| t.animation.is_some()) {
        draw::render_tile(tile, time);
    }
    for tile in tiles.iter().filter(|&t| t.animation.is_none()) {
        draw::render_tile(tile, time);
        if let Some(targeting) = &control_status.targeting {
            draw::draw_bordered_hexagon(
                &targeting.pos,
                targeting.pos.screen_size,
                &PINK, 
                0.35,
            );
        }
    }
    for object in non_tiles.iter().filter(|&t| t.animation.is_none()) {
        draw::render_non_tile_object(object, time);
    }
    for object in non_tiles.iter().filter(|&t| t.animation.is_some()) {
        draw::render_non_tile_object(object, time);
    }
    
    if let Some(dragged) = &control_status.dragging {
        let screen_coord = ScreenCoord::mouse_pos(game.board.size);
        draw::render_dragged(dragged, screen_coord, time);
    }

    set_default_camera();
    // clear_background(WHITE);
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
