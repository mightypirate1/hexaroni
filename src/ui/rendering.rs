use std::{env, fs};

use super::{control::ControlStatus, draw};
use crate::engine::Game;
use crate::geometry::ScreenCoord;
use macroquad::prelude::*;

pub struct Renderer {
    pub fg_target: RenderTarget,
    pub bg_target: RenderTarget,
    pub fg_camera: Camera2D,
    pub bg_camera: Camera2D,
    pub material: Material,
}

impl Renderer {
    pub fn new(render_scale: f32) -> Renderer {
        let fg_target = Renderer::create_target(render_scale);
        let bg_target = Renderer::create_target(render_scale);
        let path_to_crate = env!("CARGO_MANIFEST_DIR");
        Renderer {
            fg_target: fg_target.clone(),
            bg_target: bg_target.clone(),
            fg_camera: Renderer::create_camera(&fg_target),
            bg_camera: Renderer::create_camera(&bg_target),
            material: load_material(
                ShaderSource::Glsl {
                    vertex: &fs::read_to_string(format!(
                        "{}/src/ui/shaders/vertex.glsl",
                        path_to_crate
                    ))
                    .expect("unable to load vertex shader"),
                    fragment: &fs::read_to_string(format!(
                        "{}/src/ui/shaders/frag.glsl",
                        path_to_crate
                    ))
                    .expect("unable to load fragment shader"),
                },
                MaterialParams {
                    uniforms: vec![UniformDesc::new("canvasSize", UniformType::Float2)],
                    ..Default::default()
                },
            )
            .unwrap(),
        }
    }

    pub fn render(&self, game: &Game, control_status: &ControlStatus, time: f32) {
        // initialize bg rendering
        gl_use_material(&self.material);
        self.material
            .set_uniform("canvasSize", (screen_width(), screen_height()));
        set_camera(&self.bg_camera);
        // render bg
        Renderer::draw_texture_from_target(&self.bg_target);

        // initialize and render fg
        gl_use_default_material();
        set_camera(&self.fg_camera);
        self.render_fg(game, control_status, time);
    }

    fn render_fg(&self, game: &Game, control_status: &ControlStatus, time: f32) {
        // background
        Renderer::draw_texture_from_target(&self.bg_target);

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
                    draw::render_tile_status_color(tile, &SKYBLUE, time);
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
        Renderer::draw_texture_from_target(&self.fg_target);
    }

    fn draw_texture_from_target(target: &RenderTarget) {
        draw_texture_ex(
            &target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    fn create_target(render_scale: f32) -> RenderTarget {
        let render_target = render_target(
            (render_scale * screen_width()) as u32,
            (render_scale * screen_height()) as u32,
        );
        render_target.texture.set_filter(FilterMode::Linear);
        render_target
    }

    fn create_camera(render_target: &RenderTarget) -> Camera2D {
        Camera2D {
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(0.5 * screen_width(), 0.5 * screen_height()),
            render_target: Some(render_target.clone()),
            ..Default::default()
        }
    }
}
