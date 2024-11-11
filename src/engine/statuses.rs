use crate::geometry::ScreenCoord;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Selected,
    Dragged,
    Hovered,
    Targeted,
    Targetable,
    Killed {
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
        pos: ScreenCoord,
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
