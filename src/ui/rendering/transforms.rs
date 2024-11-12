use crate::engine::{statuses::Status, Object};
use crate::geometry::ScreenCoord;
use itertools::Itertools;
use macroquad::prelude::*;

pub fn create_model_matrix(object: &Object, time: f32) -> Mat4 {
    fn sort_key(status: &Status) -> i32 {
        /*
        matrix multiplication does not commute, so we attempt to
        sort the statuses in the order the corresponding matrices
        need to be applied
        */
        match status {
            Status::Wobble { .. } => 0,
            Status::Killed { .. } => 1,
            Status::Move { .. } => 1,
            _ => -1,
        }
    }
    let position = ScreenCoord::from_hexcoord(&object.coord);
    let matrices: Vec<Mat4> = object
        .statuses
        .iter()
        .filter(|s| sort_key(s) >= 0)
        .sorted_by(|s1, s2| sort_key(s1).cmp(&sort_key(s2)))
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
    match status {
        Status::Wobble {
            amplitude,
            start_time,
            speed,
        } => {
            let t = time - start_time;
            let angle = amplitude * (speed * t).sin();
            Mat4::from_rotation_x(angle)
        }
        Status::Killed {
            start_time,
            duration,
            knockback,
        } => {
            let progress = (time - start_time) / duration;
            let heaven = vec3(0.0, 0.0, -screen_height());
            let translation = heaven + *knockback;
            Mat4::from_translation(progress * translation)
        }
        Status::Move {
            from,
            to,
            start_time,
            duration,
        } => {
            /*
                since every object will eventually eventually be translated
                to their position, we will subtract the destination here.

                note: this code assumes that the `to` coord is the same as the
                object's current coord.
            */
            let progress = (time - start_time) / duration;
            let src = from.as_vec();
            let dst = to.as_vec();
            let target = src - dst + progress * (dst - src);
            let jump = f32::sin(std::f32::consts::PI * progress)
                * vec3(0.0, 0.0, -0.3 * (dst - src).length());
            Mat4::from_translation(target + jump)
        }
        _ => panic!("no status matrix implemented for {:?}", status),
    }
}
