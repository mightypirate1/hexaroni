use crate::engine::statuses::Effect;
use crate::geometry::ScreenCoord;
use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum StatusType {
    #[default]
    Selected,
    Dragged,
    Hovered,
    Targeted,
    Killed {
        knockback: Vec3,
    },
    Move {
        from: ScreenCoord,
        to: ScreenCoord,
        height: f32,
    },
    Wobble {
        amplitude: f32,
        speed: f32,
    },
    Falling,
    DelayedEffect {
        move_nr: usize,
        effect: Effect,
        indicator_move_nr: Option<usize>,
        indicator: Option<Effect>,
    },
}
