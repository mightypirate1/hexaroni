use crate::engine::{statuses::StatusType, ObjectType, Player};
use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Config {
    pub starting_player: Player,
    pub player_color: HashMap<Player, Vec4>,
    pub object_color: HashMap<ObjectType, Vec4>,
    pub game_start_countdown: f32,
    pub play_move_timeout: f32,
    pub move_application_time: f32,
    pub kill_duration: f32,
    pub tile_base_color: Vec4,
    pub tile_dragged_from_color: Vec4,
    pub tile_targeted_color: Vec4,
    pub tile_possible_move_color: Vec4,
    pub camera_up: Vec3,
    pub camera_position: Vec3,
    pub camera_target: Vec3,
    pub render_scale: f32,
    pub falling_tiles_heads_up: usize,
    pub falling_tiles_indicator: StatusType,
    pub dasher_can_fly: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            starting_player: Player::A,
            player_color: HashMap::from([
                (Player::A, PINK.to_vec()),
                (Player::B, SKYBLUE.to_vec()),
                (Player::God, BLACK.to_vec()),
            ]),
            object_color: HashMap::from([
                (ObjectType::Wall, vec4(0.06, 0.06, 0.06, 1.0)),
                (ObjectType::Dasher, BLACK.to_vec()),
                (ObjectType::Jumper, BLACK.to_vec()),
            ]),
            game_start_countdown: 2.5,
            play_move_timeout: 5.0,
            move_application_time: 0.25,
            kill_duration: 0.4,
            tile_base_color: vec4(0.03, 0.03, 0.03, 1.0),
            tile_dragged_from_color: RED.to_vec(),
            tile_targeted_color: RED.to_vec(),
            tile_possible_move_color: 0.5 * SKYBLUE.to_vec(),
            camera_up: vec3(0.0, 0.0, 1.0),
            camera_target: vec3(0.0, 0.0, 0.0),
            camera_position: -vec3(-0.5, -1.5, 10.0),
            render_scale: 1.0,
            falling_tiles_heads_up: 2,
            falling_tiles_indicator: StatusType::Wobble {
                amplitude: 0.2,
                speed: 37.1,
            },
            dasher_can_fly: false,
        }
    }
}

lazy_static! {
    pub static ref CONF: Config = Config::default();
}
