use crate::geometry::ScreenCoord;

#[derive(Copy, Clone, Debug)]
pub enum Animation {
    Kill {
        pos: ScreenCoord,
        start_time: f32,
        duration: f32,
    },
    Wobble {
        pos: ScreenCoord,
        amplitude: f32,
        start_time: f32,
        speed: f32,
    },
    Move {
        from: ScreenCoord,
        to: ScreenCoord,
        start_time: f32,
        duration: f32,
    },
}

impl Animation {
    pub fn get_pos(&self, time: f32) -> ScreenCoord {
        match self {
            Animation::Move {
                from,
                to,
                start_time,
                duration,
            } => {
                let progress = (time - start_time) / duration;
                if progress >= 1.0 {
                    *to
                } else if progress <= 0.0 {
                    *from
                } else {
                    let delta = to.sub(from);
                    from.add(&delta.scale(f32::powi(progress, 2)))
                }
            }
            Animation::Kill {
                pos,
                start_time,
                duration,
            } => {
                let progress = (time - start_time) / duration;
                let heaven = pos.with_z(1000.);
                if progress >= 1.0 {
                    heaven
                } else {
                    pos.add(&heaven.scale(progress))
                }
            }
            Animation::Wobble {
                pos,
                amplitude,
                start_time,
                speed,
            } => {
                let progress = (time - start_time) / 0.5;
                let wobble = ScreenCoord {
                    x: amplitude * (speed * progress * 2.0 * std::f32::consts::PI).sin(),
                    y: amplitude * (speed * progress * 2.1823 * std::f32::consts::PI).cos(),
                    z: 0.0,
                };
                pos.add(&wobble)
            }
        }
    }

    pub fn is_expired(&self, time: f32) -> bool {
        match self {
            Animation::Move {
                start_time,
                duration,
                ..
            }
            | Animation::Kill {
                start_time,
                duration,
                ..
            } => (time - start_time).gt(duration),
            _ => false,
        }
    }
}
