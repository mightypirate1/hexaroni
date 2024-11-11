use crate::engine::{Game, Object, ObjectType, Player};
use crate::geometry::ScreenCoord;
use crate::ui::{control::ControlStatus, rendering::meshes};
use itertools::Itertools;
use macroquad::prelude::*;
use macroquad::Error;
use miniquad::window::screen_size;
use miniquad::{CullFace, PipelineParams};
use std::{env, fs};

pub struct Renderer {
    render_scale: f32,
    render_target: RenderTarget,
    bg_camera: Camera2D,
    bg_material: Material,
    fg_material: Material,
}

impl Renderer {
    pub fn new(render_scale: f32) -> Result<Renderer, Error> {
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
                        UniformDesc::new("canvas_size", UniformType::Float2),
                        UniformDesc::new("render_scale", UniformType::Float1),
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
        game: &Game,
        camera: &Camera3D,
        control_status: &ControlStatus,
        time: f32,
    ) {
        // initialize and render background
        gl_use_material(&self.bg_material);
        self.bg_material.set_uniform("canvas_size", screen_size());
        self.bg_material
            .set_uniform("render_scale", self.render_scale);
        set_camera(&self.bg_camera); // bg_camera renders to the fg_target
        Renderer::draw_texture_from_target(&self.render_target);

        // render board
        set_camera(camera);
        gl_use_material(&self.fg_material);
        Renderer::render_game(game, control_status, time);
        set_default_camera();
        gl_use_default_material();
        Renderer::draw_texture_from_target(&self.render_target);

        // win screen
        if let Some(player) = game.winner() {
            Renderer::render_win(self, &player, time)
        }
    }

    fn render_game(game: &Game, control_status: &ControlStatus, time: f32) {
        let screen_size = ScreenCoord::screen_size(game.board.size);
        game.board
            .tiles
            .iter()
            .sorted_by(|a, b| a.coord.y.cmp(&b.coord.y))
            .for_each(|t| {
                Renderer::render_tile(t, control_status, screen_size);
            });
        game.board
            .objects
            .iter()
            .sorted_by(|a, b| a.coord.y.cmp(&b.coord.y))
            .for_each(|o| {
                let as_active = game.board.current_player == o.player;
                Renderer::render_object(o, as_active, screen_size, time);
            });
    }

    fn render_tile(t: &Object, control_status: &ControlStatus, screen_size: f32) {
        let mut color = vec4(0.1, 0.1, 0.1, 1.0);
        if let Some(drag) = &control_status.dragging {
            if drag.object.coord == t.coord {
                color += RED.to_vec();
            }
        } else if let Some(tgt) = &control_status.targeting {
            if tgt == t {
                color += RED.to_vec();
            }
        }
        if let Some(drag) = &control_status.dragging {
            if drag.has_move_to(&t.coord) {
                color += SKYBLUE.to_vec();
            }
        }
        let screen_coord = ScreenCoord::from_hexcoord(&t.coord);
        let pos = vec3(screen_coord.x, screen_coord.y, 0.0);
        let size = t.props.size * screen_size;
        let mesh = meshes::tile_hex_mesh(pos, size, &color);
        draw_mesh(&mesh);
    }

    fn render_object(o: &Object, as_active: bool, screen_size: f32, _time: f32) {
        let player_color = match o.player {
            Player::A => PINK.to_vec(),
            Player::B => SKYBLUE.to_vec(),
            Player::God => BLACK.to_vec(),
        };

        let screen_coord = ScreenCoord::from_hexcoord(&o.coord);
        let pos = vec3(screen_coord.x, screen_coord.y, 0.0);
        let size = o.props.size * screen_size;

        let mesh = match o.otype {
            ObjectType::Wall => {
                let object_color = BLACK.to_vec();
                meshes::obj_wall_mesh(pos, size, &object_color, &player_color)
            }
            ObjectType::Dasher => {
                let object_color = BLACK.to_vec();
                meshes::obj_jumper_mesh(pos, size, &object_color, &player_color, as_active)
            }
            ObjectType::Jumper => {
                let object_color = BLACK.to_vec();
                meshes::obj_dasher_mesh(pos, size, &object_color, &player_color, as_active)
            }
            _ => panic!("bad thing happen"),
        };
        draw_mesh(&mesh);
    }

    fn render_win(&self, winner: &Player, _time: f32) {
        let text = format!("{:?} rocks!", &winner);
        let (w, h) = screen_size();
        let text_width = 1.2 * h;
        draw_text(&text, 0.25 * (w - text_width), 0.5 * h, 0.5 * h, ORANGE);
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
