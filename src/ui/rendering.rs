use macroquad::prelude::*;
use crate::game::Game;
use super::draw;


pub fn render(game: &Game, time: f32, render_target: &RenderTarget) {
    let w = screen_width();
    let h = screen_height();
    set_camera(&Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        target: vec2(0.5 * w, 0.5 * h),
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    draw::render_game(game, time);    
    
    set_default_camera();
    clear_background(WHITE);
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
