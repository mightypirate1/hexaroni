use super::{HexCoord, ObjectType};
use crate::renderer::traits::Animation;

#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub coord: HexCoord,
    pub animation: Animation,
}

impl Object {
    pub fn new(otype: ObjectType, coord: HexCoord) -> Object {
        Object {
            otype,
            coord,
            animation: Animation::Wobble {
                coord,
                start_time: 0.0,
                amplitude: 5.0,
            },
        }
    }
    
}