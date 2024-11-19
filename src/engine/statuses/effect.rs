use crate::config::CONF;
use crate::engine::{
    statuses::{Status, StatusType},
    Object,
};
use crate::geometry::{HexCoord, ScreenCoord};
use macroquad::prelude::*;

/**
`Effect`s are sent by other entities to the `GameController` for it to apply.
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Kill {
        victim: Object,
        killer: Option<Object>,
        /// at what stage of the move-animation the kill occurs
        animation_delay_frac: Option<f32>,
    },
    KillAallOn {
        coord: HexCoord,
        apply: Option<Box<StatusType>>,
        duration: Option<f32>,
    },
    SetStatus {
        object: Object,
        stype: Box<StatusType>,
        duration: Option<f32>,
    },
    NoOp,
}

impl Effect {
    /**
    returns any Status that should be applied to the things the effect will apply to
    */
    pub fn applying_status(&self, time: f32) -> Option<Status> {
        match self {
            Effect::Kill {
                victim,
                killer,
                animation_delay_frac,
            } => {
                let knockback = if let Some(k) = killer {
                    let killer_coord = ScreenCoord::from_hexcoord(&k.coord);
                    let obj_coord = ScreenCoord::from_hexcoord(&victim.coord);
                    obj_coord.as_vec() - killer_coord.as_vec()
                } else {
                    vec3(0.0, 0.0, 0.0)
                };
                let duration = CONF.kill_duration;
                let start_time = time + duration * animation_delay_frac.unwrap_or(0.0);
                let status = Status::new_killed(knockback, start_time, duration);
                Some(status)
            }
            Effect::KillAallOn {
                apply, duration, ..
            } => {
                if let Some(applied) = apply {
                    let status = Status {
                        stype: *applied.clone(),
                        start_time: Some(time),
                        duration: *duration,
                    };
                    Some(status)
                } else {
                    None
                }
            }
            Effect::SetStatus {
                stype, duration, ..
            } => {
                let status = Status {
                    stype: *stype.clone(),
                    start_time: Some(time),
                    duration: *duration,
                };
                Some(status)
            }
            Effect::NoOp => None,
        }
    }
}
