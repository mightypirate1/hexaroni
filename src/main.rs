use hexaroni::config::CONF;
use hexaroni::engine::statuses::StatusType;
use hexaroni::game::GameController;
use hexaroni::ui::{
    control::{ControlStatus, KbdAction, MouseAction},
    rendering::Renderer,
    Drag,
};
use macroquad::prelude::*;
use miniquad::window::screen_size;
use std::time::Instant;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hexaroni".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameController::new();
    let mut control_status = ControlStatus::default();
    let mut renderer = Renderer::new().unwrap();
    let mut curr_window_size = screen_size();
    let camera_up = CONF.camera_up;
    let camera_target = CONF.camera_target;
    let mut camera_position = CONF.camera_position;
    let start_time = Instant::now();

    loop {
        // recreate shader on resize
        if curr_window_size != screen_size() {
            match Renderer::new() {
                Ok(r) => renderer = r,
                Err(msg) => println!("{}", msg),
            };
            curr_window_size = screen_size();
        }

        // update control, camera, and game state
        let curr_time = start_time.elapsed().as_secs_f32();
        game.tick(curr_time);
        camera_position = control_camera(&camera_position);
        let camera = renderer.create_camera(camera_position, camera_target, camera_up);
        control_status.update(&game, &camera);

        // handle events
        match &control_status.action {
            MouseAction::Dragging => {
                if control_status.dragging.is_none() {
                    // if we are not dragging already, we set the hovered object to dragging
                    if let Some(ref hovered) = control_status.hovering {
                        if hovered.props.draggable
                            && hovered.owned_by(&game.current_player())
                            && !hovered.props.dead
                        {
                            control_status.dragging = Some(Drag::create(hovered, &mut game));
                        }
                    }
                }
            }
            MouseAction::Drop => {
                if let Some(drag) = &control_status.dragging {
                    game.board
                        .get_as_mut(&drag.object)
                        .unwrap()
                        .remove_status(&StatusType::Dragged);
                    if let Some(target_tile) = &control_status.targeting {
                        if let Some(r#move) = drag.get_move_to(&target_tile.coord) {
                            game.apply_move(r#move, curr_time, CONF.move_application_time);
                        }
                    }
                }
                control_status.dragging = None;
            }
            _ => {}
        }

        match get_event() {
            Some(KbdAction::StartGame) => game.start_game(),
            Some(KbdAction::Reset) => game = GameController::new(),
            Some(KbdAction::Quit) => break,
            Some(KbdAction::ReloadShader) => {
                match Renderer::new() {
                    Ok(r) => renderer = r,
                    Err(msg) => println!("{}", msg),
                };
            }
            _ => {}
        }

        renderer.render(&game, &camera, &control_status, curr_time);
        next_frame().await;
    }
}

fn get_event() -> Option<KbdAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(KbdAction::Quit);
    }
    if is_key_pressed(KeyCode::Space) {
        return Some(KbdAction::ReloadShader);
    }
    if is_key_pressed(KeyCode::Enter) {
        return Some(KbdAction::StartGame);
    }
    if is_key_pressed(KeyCode::R) {
        return Some(KbdAction::Reset);
    }
    None
}

fn control_camera(camera_position: &Vec3) -> Vec3 {
    let mut camera_position = *camera_position;
    if is_key_down(KeyCode::LeftAlt) {
        let step = 0.99;
        if is_key_down(KeyCode::Up) {
            camera_position *= step;
        }
        if is_key_down(KeyCode::Down) {
            camera_position /= step;
        }
    } else {
        if is_key_down(KeyCode::Up) {
            let orth = vec3(camera_position.y, -camera_position.x, 0.0).normalize();
            camera_position = Mat4::from_axis_angle(orth, 0.01).project_point3(camera_position);
        }
        if is_key_down(KeyCode::Down) {
            let orth = vec3(camera_position.y, -camera_position.x, 0.0).normalize();
            camera_position = Mat4::from_axis_angle(orth, -0.01).project_point3(camera_position);
        }
        if is_key_down(KeyCode::Left) {
            camera_position = Mat4::from_rotation_z(0.01).project_point3(camera_position);
        }
        if is_key_down(KeyCode::Right) {
            camera_position = Mat4::from_rotation_z(-0.01).project_point3(camera_position);
        }
    }
    camera_position
}
