use crate::engine::Object;
use crate::ui::rendering::{transforms, Renderable};
use itertools::izip;
use macroquad::prelude::*;

/**
 Assumes use of default camera.

 All arguments are in fractions of screen size.
*/
pub fn hud_quad(x0: f32, y0: f32, x1: f32, y1: f32) -> Mesh {
    let x0 = screen_width() * x0;
    let x1 = screen_width() * x1;
    let y0 = screen_height() * y0;
    let y1 = screen_height() * y1;
    Mesh {
        vertices: vec![
            Vertex {
                position: vec3(x0, y0, 0.0),
                uv: vec2(0.0, 0.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(x1, y0, 0.0),
                uv: vec2(1.0, 0.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(x0, y1, 0.0),
                uv: vec2(0.0, 1.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(x0, y1, 0.0),
                uv: vec2(0.0, 1.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(x1, y0, 0.0),
                uv: vec2(1.0, 0.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(x1, y1, 0.0),
                uv: vec2(1.0, 1.0),
                color: BLACK.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0),
            },
        ],
        indices: vec![0, 1, 2, 3, 4, 5],
        texture: None,
    }
}

pub fn tile_hex_mesh(tile: &Object, color: &Vec4, as_highlighted: bool, time: f32) -> Renderable {
    let model_matrix = transforms::create_model_matrix(tile, time);
    let size = tile.props.size;
    let d = 0.86602;
    let thickness = 0.2 * size;
    let glow = if as_highlighted { 1.0 } else { 0.0 };

    let position = model_matrix.project_point3(vec3(0.0, 0.0, 0.0));
    let offsets = [
        vec3(0.0, 1.0, 0.0),
        vec3(-d, 0.5, 0.0),
        vec3(-d, -0.5, 0.0),
        vec3(0.0, -1.0, 0.0),
        vec3(d, -0.5, 0.0),
        vec3(d, 0.5, 0.0),
    ];
    let corner_positions: Vec<Vec3> = offsets.iter().map(|&offset| size * offset).collect();
    let center_vertex = [Vertex {
        position,
        uv: vec2(0.0, 1.0),
        color: Color::from_vec(*color).into(),
        normal: vec3(0.0, 0.0, -1.0).normalize().extend(glow),
    }];
    let corner_vertices: Vec<Vertex> = corner_positions
        .iter()
        .map(|cp| Vertex {
            position: model_matrix.project_point3(*cp),
            uv: vec2(1.0, 1.0),
            color: Color::from_vec(*color).into(),
            normal: cp.with_z(-1.0).normalize().extend(glow),
        })
        .collect();
    let bottom_corner_vertices: Vec<Vertex> = izip!(&offsets, &corner_positions)
        .map(|(o, cp)| Vertex {
            position: model_matrix.project_point3(*cp + vec3(0.0, 0.0, thickness)),
            uv: vec2(1.0, 0.0),
            color: Color::from_vec(*color).into(),
            normal: o.normalize().extend(glow),
        })
        .collect();

    let indices = vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1, 10, 11, 5, 11, 12, 6, 12, 7, 1, 7, 8,
        2, 8, 9, 3, 9, 10, 4, 5, 4, 10, 6, 5, 11, 1, 6, 12, 2, 1, 7, 3, 2, 8, 4, 3, 9,
    ];
    let vertices = center_vertex
        .iter()
        .chain(&corner_vertices)
        .chain(&bottom_corner_vertices)
        .copied()
        .collect();

    Renderable {
        mesh: Mesh {
            vertices,
            indices,
            texture: None,
        },
        position,
    }
}

pub fn obj_wall_mesh(
    object: &Object,
    object_color: &Vec4,
    player_color: &Vec4,
    time: f32,
) -> Renderable {
    let model_matrix = transforms::create_model_matrix(object, time);
    let size = object.props.size;
    let d = 0.71;

    let position = model_matrix.project_point3(vec3(0.0, 0.0, 0.0));
    let offsets = [
        vec3(d, 0.0, 0.0),
        vec3(0.0, d, 0.0),
        vec3(-d, 0.0, 0.0),
        vec3(0.0, -d, 0.0),
    ];
    let bottom_vertices: Vec<Vertex> = offsets
        .iter()
        .map(|o| Vertex {
            position: model_matrix.project_point3(size * (*o)),
            uv: vec2(0.0, 0.0),
            normal: o.normalize().extend(0.0),
            color: Color::from_vec(*object_color).into(),
        })
        .collect();
    let top_vertices: Vec<Vertex> = offsets
        .iter()
        .map(|o| Vertex {
            position: model_matrix.project_point3(size * o.with_z(-d)),
            uv: vec2(1.0, 1.0),
            normal: o.with_z(-d).normalize().extend(0.0),
            color: Color::from_vec(*object_color).into(),
        })
        .collect();

    let vertices = bottom_vertices
        .iter()
        .chain(&top_vertices)
        .copied()
        .collect();
    let indices = vec![
        0, 1, 4, 1, 2, 5, 2, 3, 6, 3, 0, 7, 0, 4, 7, 1, 5, 4, 2, 6, 5, 3, 7, 6, 4, 5, 7, 6, 7, 5,
    ];
    let texture = texture_from_2_colors(object_color, player_color);

    Renderable {
        mesh: Mesh {
            vertices,
            indices,
            texture: Some(texture),
        },
        position,
    }
}

pub fn obj_jumper_mesh(
    object: &Object,
    object_color: &Vec4,
    player_color: &Vec4,
    as_active: bool,
    time: f32,
) -> Renderable {
    let model_matrix = transforms::create_model_matrix(object, time);
    let size = object.props.size;
    let t = 0.25 * 3.0_f32.sqrt();
    let d = 0.5 / 3.0_f32.sqrt();

    let position = model_matrix.project_point3(vec3(0.0, 0.0, 0.0));
    let offsets = [vec3(0.5, d, 0.0), vec3(-0.5, d, 0.0), vec3(0.0, -t, 0.0)];
    let top_vertex = [Vertex {
        position: model_matrix.project_point3(size * vec3(0.0, 0.0, -1.5)),
        uv: vec2(if as_active { 1.2 } else { 0.7 }, 0.0),
        color: Color::from_vec(*object_color).into(),
        normal: vec3(0.0, 0.0, -1.0).normalize().extend(0.0),
    }];
    let bottom_vertices: Vec<Vertex> = offsets
        .iter()
        .map(|o| Vertex {
            position: model_matrix.project_point3(size * (*o)),
            uv: vec2(0.0, 0.0),
            color: Color::from_vec(*object_color).into(),
            normal: o.normalize().extend(0.0),
        })
        .collect();
    let vertices = bottom_vertices.iter().chain(&top_vertex).copied().collect();
    let indices = vec![0, 1, 3, 1, 2, 3, 2, 0, 3];
    let texture = texture_from_2_colors(object_color, player_color);

    Renderable {
        mesh: Mesh {
            vertices,
            indices,
            texture: Some(texture),
        },
        position,
    }
}

pub fn obj_dasher_mesh(
    object: &Object,
    object_color: &Vec4,
    player_color: &Vec4,
    as_active: bool,
    time: f32,
) -> Renderable {
    let model_matrix = transforms::create_model_matrix(object, time);
    let size = object.props.size;
    let d = 0.4;

    let position = model_matrix.project_point3(vec3(0.0, 0.0, 0.0));
    let offsets = [
        vec3(d, d, 0.0),
        vec3(-d, d, 0.0),
        vec3(-d, -d, 0.0),
        vec3(d, -d, 0.0),
    ];

    let bottom_vertices: Vec<Vertex> = offsets
        .iter()
        .map(|o| Vertex {
            position: model_matrix.project_point3(size * (*o)),
            uv: vec2(0.0, 0.0),
            normal: o.extend(0.0).normalize(),
            color: Color::from_vec(*object_color).into(),
        })
        .collect();
    let middle_vertices: Vec<Vertex> = offsets
        .iter()
        .map(|o| Vertex {
            position: model_matrix.project_point3(size * o.with_z(-d)),
            uv: vec2(if as_active { 0.1 } else { 0.0 }, 0.0),
            normal: vec4(o.x, o.y, d, 1.0).normalize(),
            color: Color::from_vec(*object_color).into(),
        })
        .collect();
    let top_vertex = [Vertex {
        position: model_matrix.project_point3(size * vec3(0.0, 0.0, -2.0 * d)),
        normal: vec4(0.0, 0.0, -1.0, 1.0).normalize(),
        uv: vec2(if as_active { 1.0 } else { 0.6 }, 0.0),
        color: Color::from_vec(*object_color).into(),
    }];

    let vertices = bottom_vertices
        .iter()
        .chain(&middle_vertices)
        .chain(&top_vertex)
        .copied()
        .collect();
    let indices = vec![
        0, 1, 4, 1, 2, 5, 2, 3, 6, 3, 0, 7, 0, 4, 7, 1, 5, 4, 2, 6, 5, 3, 7, 6, 4, 5, 7, 6, 7, 5,
        4, 5, 8, 5, 6, 8, 6, 7, 8, 7, 4, 8,
    ];
    let texture = texture_from_2_colors(object_color, player_color);

    Renderable {
        mesh: Mesh {
            vertices,
            indices,
            texture: Some(texture),
        },
        position,
    }
}

fn texture_from_2_colors(color_a: &Vec4, color_b: &Vec4) -> Texture2D {
    let col_a = color_to_bytes(color_a);
    let col_b = color_to_bytes(color_b);
    let bytes: &[u8] = &[
        col_a[0], col_a[1], col_a[2], col_a[3], col_b[0], col_b[1], col_b[2], col_b[3],
    ];
    Texture2D::from_rgba8(2, 1, bytes)
}

fn color_to_bytes(color: &Vec4) -> [u8; 4] {
    let uvec = (*color * 255.0).as_uvec4();
    [uvec.x as u8, uvec.y as u8, uvec.z as u8, uvec.w as u8]
}
