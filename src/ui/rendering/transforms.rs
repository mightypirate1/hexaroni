use crate::engine::{
    statuses::{Status, StatusType},
    Object,
};
use crate::geometry::ScreenCoord;
use itertools::Itertools;
use macroquad::prelude::*;

pub fn create_model_matrix(object: &Object, time: f32) -> Mat4 {
    fn sort_key(stype: &StatusType) -> i32 {
        /*
        matrix multiplication does not commute, so we attempt to
        sort the statuses in the order the corresponding matrices
        need to be applied
        */
        match stype {
            StatusType::Wobble { .. } => 0,
            StatusType::Killed { .. } => 1,
            StatusType::Move { .. } => 1,
            StatusType::Falling { .. } => 1,
            _ => -1,
        }
    }
    let position = ScreenCoord::from_hexcoord(&object.coord);
    let matrices: Vec<Mat4> = object
        .statuses
        .iter()
        .filter(|s| sort_key(&s.stype) >= 0)
        .sorted_by(|s1, s2| sort_key(&s1.stype).cmp(&sort_key(&s2.stype)))
        .map(|s| status_matrix(s, time))
        .collect();

    let mut model_matrix = Mat4::IDENTITY;
    for matrix in matrices {
        model_matrix = matrix.mul_mat4(&model_matrix);
    }
    let position_matrix = Mat4::from_translation(position.as_vec());

    position_matrix.mul_mat4(&model_matrix)
}

fn status_matrix(status: &Status, time: f32) -> Mat4 {
    match status.stype {
        StatusType::Wobble { amplitude, speed } => {
            let start_time = status.start_time.expect("wobble without start_time");
            let t = time - start_time;
            let angle = amplitude * (speed * t).sin();
            Mat4::from_rotation_x(angle)
        }
        StatusType::Killed { knockback } => {
            let start_time = status.start_time.expect("killed without start_time");
            let duration = status.duration.expect("killed without duration");
            let progress = ((time - start_time) / duration).clamp(0.0, 1.0);
            let heaven = vec3(0.0, 0.0, -7.0);
            let translation = heaven + 1.5 * knockback;
            Mat4::from_translation(progress * translation)
        }
        StatusType::Move { from, to, height } => {
            /*
                since every object will eventually eventually be translated
                to their position, we will subtract the destination here.

                note: this code assumes that the `to` coord is the same as the
                object's current coord.
            */
            let start_time = status.start_time.expect("move without start_time");
            let duration = status.duration.expect("move without duration");
            let progress = ((time - start_time) / duration).clamp(0.0, 1.0);
            let src = from.as_vec();
            let dst = to.as_vec();
            let target = src - dst + progress * (dst - src);
            let jump = progress * (progress - 1.0) * vec3(0.0, 0.0, height) * (dst - src).length();
            Mat4::from_translation(target + jump)
        }
        StatusType::Falling => {
            let start_time = status.start_time.expect("falling without start_time");
            const DOWN: Vec3 = vec3(0.0, 0.0, 1.0);
            let t = time - start_time;
            let g = 2.7;
            Mat4::from_translation(g * DOWN * t * t)
        }
        _ => panic!("no status matrix implemented for {:?}", status),
    }
}
