use crate::engine::{Object, ObjectType, Player};
use crate::ui::control::ControlStatus;
use crate::ui::rendering::meshes;
use macroquad::prelude::*;

pub struct Renderable {
    pub mesh: Mesh,
    pub position: Vec3,
}

impl Renderable {
    pub fn from_tile(
        tile: &Object,
        control_status: &ControlStatus,
        screen_size: f32,
        time: f32,
    ) -> Renderable {
        let mut color = vec4(0.1, 0.1, 0.1, 1.0);
        if let Some(drag) = &control_status.dragging {
            if drag.object.coord == tile.coord {
                color += RED.to_vec();
            }
        } else if let Some(tgt) = &control_status.targeting {
            if tgt == tile {
                color += RED.to_vec();
            }
        }
        if let Some(drag) = &control_status.dragging {
            if drag.has_move_to(&tile.coord) {
                color += SKYBLUE.to_vec();
            }
        }
        meshes::tile_hex_mesh(tile, &color, screen_size, time)
    }

    pub fn from_object(
        object: &Object,
        as_active: bool,
        screen_size: f32,
        time: f32,
    ) -> Renderable {
        let player_color = match object.player {
            Player::A => PINK.to_vec(),
            Player::B => SKYBLUE.to_vec(),
            Player::God => BLACK.to_vec(),
        };

        match object.otype {
            ObjectType::Wall => {
                let object_color = vec4(0.02, 0.02, 0.02, 1.0);
                meshes::obj_wall_mesh(object, &object_color, &object_color, screen_size, time)
            }
            ObjectType::Dasher => {
                let object_color = BLACK.to_vec();
                meshes::obj_jumper_mesh(
                    object,
                    &object_color,
                    &player_color,
                    as_active,
                    screen_size,
                    time,
                )
            }
            ObjectType::Jumper => {
                let object_color = BLACK.to_vec();
                meshes::obj_dasher_mesh(
                    object,
                    &object_color,
                    &player_color,
                    as_active,
                    screen_size,
                    time,
                )
            }
            _ => panic!("bad thing happen"),
        }
    }
}
