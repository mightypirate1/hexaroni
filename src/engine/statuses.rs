use crate::geometry::ScreenCoord;
use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Selected,
    Dragged,
    Hovered,
    Targeted,
    Targetable,
    Killed {
        knockback: Vec3,
        start_time: f32,
        duration: f32,
    },
    Move {
        from: ScreenCoord,
        to: ScreenCoord,
        start_time: f32,
        duration: f32,
    },
    Wobble {
        amplitude: f32,
        start_time: f32,
        speed: f32,
    },
}

impl Status {
    pub fn is_expired(&self, time: f32) -> bool {
        match self {
            Status::Killed {
                start_time,
                duration,
                ..
            } => start_time + duration < time,
            Status::Move {
                start_time,
                duration,
                ..
            } => start_time + duration < time,
            _ => false,
        }
    }
}
