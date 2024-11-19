use crate::engine::statuses::{Effect, StatusType};
use crate::geometry::ScreenCoord;
use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Status {
    pub stype: StatusType,
    pub start_time: Option<f32>,
    pub duration: Option<f32>,
}

impl Status {
    pub fn new(stype: StatusType) -> Self {
        Status {
            stype,
            start_time: None,
            duration: None,
        }
    }

    pub fn new_move(
        from: ScreenCoord,
        to: ScreenCoord,
        start_time: f32,
        duration: f32,
        height: f32,
    ) -> Self {
        Status {
            stype: StatusType::Move { from, to, height },
            start_time: Some(start_time),
            duration: Some(duration),
        }
    }

    pub fn new_killed(knockback: Vec3, start_time: f32, duration: f32) -> Self {
        Status {
            stype: StatusType::Killed { knockback },
            start_time: Some(start_time),
            duration: Some(duration),
        }
    }

    pub fn new_dragged() -> Self {
        Status {
            stype: StatusType::Dragged,
            start_time: None,
            duration: None,
        }
    }

    pub fn new_delayed_effect(move_nr: usize, effect: Effect) -> Status {
        Status::new(StatusType::DelayedEffect {
            move_nr,
            effect,
            indicator_move_nr: None,
            indicator: None,
        })
    }

    pub fn new_delayed_effect_with_indicator(
        move_nr: usize,
        effect: Effect,
        indicator_move_nr: usize,
        indicator: Effect,
    ) -> Status {
        Status::new(StatusType::DelayedEffect {
            move_nr,
            effect,
            indicator_move_nr: Some(indicator_move_nr),
            indicator: Some(indicator),
        })
    }

    pub fn restarted_at(&self, time: f32) -> Status {
        if self.start_time.is_none() {
            panic!("restarted {:?} which has no start_time", self);
        }
        Status {
            stype: self.stype.clone(),
            start_time: Some(time),
            duration: self.duration,
        }
    }

    pub fn with_times(&self, start_time: f32, duration: f32) -> Status {
        Status {
            stype: self.stype.clone(),
            start_time: Some(start_time),
            duration: Some(duration),
        }
    }

    pub fn is_expired(&self, time: f32) -> bool {
        if let Some(start_time) = self.start_time {
            if let Some(duration) = self.duration {
                return time > start_time + duration;
            }
        }
        false
    }
}
