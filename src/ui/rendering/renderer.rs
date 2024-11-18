use crate::config::CONF;
use crate::engine::Player;
use crate::game::{GameController, GameState};
use crate::ui::rendering::meshes::hud_quad;
use crate::ui::{control::ControlStatus, rendering::Renderable};
use itertools::Itertools;
use macroquad::prelude::*;
use macroquad::Error;
use miniquad::window::screen_size;
use miniquad::{CullFace, PipelineParams};
use std::time::Instant;
use std::{env, fs};

pub struct Renderer {
    render_scale: f32,
    render_target: RenderTarget,
    bg_camera: Camera2D,
    bg_material: Material,
    fg_material: Material,
    hud_material: Material,
}

impl Renderer {
    pub fn new() -> Result<Renderer, Error> {
        let render_scale = CONF.render_scale;
        let render_target = Renderer::create_target(render_scale);
        Ok(Renderer {
            render_scale,
            render_target: render_target.clone(),
            bg_camera: Renderer::create_internal_camera(&render_target),
            bg_material: Renderer::fetch_material(
                "bg",
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("canvas_size", UniformType::Float2),
                        UniformDesc::new("render_scale", UniformType::Float1),
                    ],
                    ..Default::default()
                },
            )?,
            fg_material: Renderer::fetch_material(
                "fg",
                MaterialParams {
                    pipeline_params: PipelineParams {
                        cull_face: CullFace::Front,
                        ..Default::default()
                    },
                    uniforms: vec![
                        UniformDesc::new("light_pos", UniformType::Float3),
                        UniformDesc::new("cam_pos", UniformType::Float3),
                    ],
                    ..Default::default()
                },
            )?,
            hud_material: Renderer::fetch_material(
                "hud",
                MaterialParams {
                    pipeline_params: PipelineParams {
                        cull_face: CullFace::Front,
                        ..Default::default()
                    },
                    uniforms: vec![
                        UniformDesc::new("canvas_size", UniformType::Float2),
                        UniformDesc::new("render_scale", UniformType::Float1),
                        UniformDesc::new("frac_remaining", UniformType::Float1),
                        UniformDesc::new("fill_color", UniformType::Float3),
                        UniformDesc::new("flipped", UniformType::Float1),
                    ],
                    ..Default::default()
                },
            )?,
        })
    }

    pub fn create_camera(
        &self,
        camera_position: Vec3,
        camera_target: Vec3,
        camera_up: Vec3,
    ) -> Camera3D {
        Camera3D {
            position: camera_position,
            target: camera_target,
            up: camera_up,
            render_target: Some(self.render_target.clone()),
            projection: Projection::Perspective,
            ..Default::default()
        }
    }

    pub fn render(
        &mut self,
        game: &GameController,
        camera: &Camera3D,
        control_status: &ControlStatus,
        time: f32,
    ) {
        // render background
        gl_use_material(&self.bg_material);
        self.bg_material.set_uniform("canvas_size", screen_size());
        self.bg_material
            .set_uniform("render_scale", self.render_scale);
        set_camera(&self.bg_camera); // bg_camera renders to the fg_target
        Renderer::draw_texture_from_target(&self.render_target);

        // render board
        set_camera(camera);
        gl_use_material(&self.fg_material);
        let light_pos = camera.position; // same as camera in cam space
        let light_pos = Mat4::from_rotation_z(time).project_point3(light_pos);
        self.fg_material.set_uniform("light_pos", light_pos);
        self.fg_material.set_uniform("cam_pos", camera.position);
        Renderer::render_game(game, camera, control_status, time);

        // render to screen
        set_default_camera();
        if let GameState::Playing {
            current_player,
            move_start,
            ..
        } = game.game_state
        {
            // render hud
            let time_remaining = CONF.play_move_timeout - move_start.elapsed().as_secs_f32();
            let frac_remaining = time_remaining / CONF.play_move_timeout;
            let flipped: f32 = if current_player == Player::A { 0. } else { 1. };
            gl_use_material(&self.hud_material);
            self.hud_material.set_uniform("canvas_size", screen_size());
            self.hud_material
                .set_uniform("render_scale", self.render_scale);
            self.hud_material
                .set_uniform("frac_remaining", frac_remaining);
            self.hud_material.set_uniform("flipped", flipped);
            self.hud_material.set_uniform(
                "fill_color",
                CONF.player_color.get(&current_player).unwrap().xyz(),
            );
            let mesh = hud_quad(0.85, 0.85, 0.98, 0.98);
            draw_mesh(&mesh);
        }
        gl_use_default_material();
        Renderer::draw_texture_from_target(&self.render_target);

        // status text
        match game.game_state {
            GameState::Waiting => {
                Renderer::render_waiting(time);
            }
            GameState::GameOver { winner } => {
                Renderer::render_win(&winner, time);
            }
            GameState::Countdown { started_at } => {
                Renderer::render_countdown(&started_at, CONF.game_start_countdown, time);
            }
            _ => {}
        }
    }

    fn render_game(
        game: &GameController,
        camera: &Camera3D,
        control_status: &ControlStatus,
        time: f32,
    ) {
        let tile_renderables: Vec<Renderable> = game
            .board
            .tiles()
            .iter()
            .map(|t| Renderable::from_tile(t, control_status, time))
            .collect();
        let piece_renderables: Vec<Renderable> = game
            .board
            .pieces()
            .iter()
            .map(|o| {
                let as_active = game.current_player() == o.player;
                Renderable::from_object(o, as_active, time)
            })
            .collect();
        tile_renderables
            .iter()
            .chain(&piece_renderables)
            .sorted_by(|a, b| {
                f32::total_cmp(
                    &(b.position - camera.position).length(),
                    &(a.position - camera.position).length(),
                )
            })
            .for_each(|renderable| draw_mesh(&renderable.mesh));
    }

    fn render_win(winner: &Player, _time: f32) {
        let text = format!("{:?} rocks!", &winner);
        let (w, h) = screen_size();
        let text_width = 0.8 * h;
        draw_text(&text, 0.15 * (w - text_width), 0.5 * h, 0.5 * h, ORANGE);
    }

    fn render_waiting(_time: f32) {
        let text = "Press enter to play.";
        let (w, h) = screen_size();
        let text_width = 1.2 * h;
        draw_text(text, 0.25 * (w - text_width), 0.5 * h, 0.15 * h, ORANGE);
    }

    fn render_countdown(started_at: &Instant, duration: f32, _time: f32) {
        let remaining = duration - started_at.elapsed().as_secs_f32();
        let scale = 1.0 * (1.0 - remaining + remaining.trunc());
        let text = format!("{}", remaining as i32);
        let (w, h) = screen_size();
        let text_width = 0.2 * h;
        draw_text(&text, 0.35 * (w - text_width), 0.65 * h, scale * h, ORANGE);
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

    fn create_internal_camera(render_target: &RenderTarget) -> Camera2D {
        Camera2D {
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(0.5 * screen_width(), 0.5 * screen_height()),
            render_target: Some(render_target.clone()),
            ..Default::default()
        }
    }

    fn fetch_material(name: &str, material_params: MaterialParams) -> Result<Material, Error> {
        let path_to_crate = env!("CARGO_MANIFEST_DIR");
        let vertex_shader_code = fs::read_to_string(format!(
            "{}/src/ui/shaders/{}/vertex.glsl",
            path_to_crate, name
        ))
        .expect("unable to load vertex shader");
        let frag_shader_code = fs::read_to_string(format!(
            "{}/src/ui/shaders/{}/frag.glsl",
            path_to_crate, name
        ))
        .expect("unable to load fragment shader");
        load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader_code,
                fragment: &frag_shader_code,
            },
            material_params,
        )
    }
}
