use crate::config::CONF;
use crate::engine::{Object, ObjectType};
use crate::ui::control::ControlStatus;
use crate::ui::rendering::meshes;
use macroquad::prelude::*;

pub struct Renderable {
    pub mesh: Mesh,
    pub position: Vec3,
}

impl Renderable {
    pub fn from_tile(tile: &Object, control_status: &ControlStatus, time: f32) -> Renderable {
        let mut as_highlighted = false;
        let mut color = CONF.tile_base_color;
        if let Some(drag) = &control_status.dragging {
            if drag.object.coord == tile.coord {
                color += CONF.tile_dragged_from_color;
                as_highlighted = true;
            }
        } else if let Some(tgt) = &control_status.targeting {
            if tgt == tile {
                color += CONF.tile_targeted_color;
                as_highlighted = true;
            }
        }
        if let Some(drag) = &control_status.dragging {
            if drag.has_move_to(&tile.coord) {
                color += CONF.tile_possible_move_color;
                as_highlighted = true;
            }
        }
        meshes::tile_hex_mesh(tile, &color, as_highlighted, time)
    }

    pub fn from_object(object: &Object, as_active: bool, time: f32) -> Renderable {
        let player_color = CONF.player_color.get(&object.player).unwrap();
        let object_color = CONF.object_color.get(&object.otype).unwrap();

        match object.otype {
            ObjectType::Wall => meshes::obj_wall_mesh(object, player_color, player_color, time),
            ObjectType::Dasher => {
                meshes::obj_dasher_mesh(object, object_color, player_color, as_active, time)
            }
            ObjectType::Jumper => {
                meshes::obj_jumper_mesh(object, object_color, player_color, as_active, time)
            }
            _ => panic!("bad thing happen"),
        }
    }
}
