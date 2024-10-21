use crate::renderer::ScreenCoord;
use crate::game::HexCoord;


#[derive(Copy, Clone, Debug)]
pub enum  Animation {
    Idle {coord: HexCoord},
    Wobble {coord: HexCoord, amplitude: f32, start_time: f32},
    Move {
        from: HexCoord,
        to: HexCoord,
        start_time: f32,
        duration: f32,
    },
}

impl Animation {
    pub fn get_coord(&self, time: f32) -> ScreenCoord {
        match self {
            Animation::Idle { coord} => ScreenCoord::from_hexcoord(coord),
            Animation::Move { from, to, start_time, duration } => {
                let progress = (time - start_time) / duration;
                let from_coord = ScreenCoord::from_hexcoord(from);
                let to_coord = ScreenCoord::from_hexcoord(to);
                if progress >= 1.0 {
                    to_coord
                } else if progress <= 0.0 {
                    from_coord
                } else {
                    let delta = to_coord.sub(&from_coord);
                    from_coord.add(&delta.scale(progress))
                }
            },
            Animation::Wobble { coord, amplitude, start_time } => {
                let progress = (time - start_time) / 0.5;
                let screen_coord = ScreenCoord::from_hexcoord(coord);
                let wobble = ScreenCoord {
                    x: amplitude * (progress * 2.0 * std::f32::consts::PI).sin(),
                    y: amplitude * (progress * 2.1823 * std::f32::consts::PI).cos(),
                    screen_size: screen_coord.screen_size,
                };
                screen_coord.add(&wobble)
            },
        }
    }
}
