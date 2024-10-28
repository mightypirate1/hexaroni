use std::time::Instant;
use macroquad::prelude::*;
use hexaroni::game::{Game, statuses::Status};
use hexaroni::ui::{rendering, control::{ControlStatus, MouseAction, KbdAction}};
use hexaroni::geometry::ScreenCoord;


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
    let render_scale = 4.0;
    let render_target = render_target(
        (render_scale * screen_width()) as u32,
        (render_scale * screen_height()) as u32,
    );
    render_target.texture.set_filter(FilterMode::Linear);
    // render_target.texture.set_filter(FilterMode::Nearest);

    let start_time = Instant::now();
    let mut game = Game::new();
    let mut control_status = ControlStatus {
        mouse_pos: ScreenCoord::new(0.0, 0.0, game.board.size),
        action: MouseAction::None,
        hovering: None,
        dragging: None,
        targeting: None,
    };

    loop {
        let curr_time = start_time.elapsed().as_secs_f32();
        game.on_tick_start(curr_time);
        
        // read mouse status
        control_status.mouse_pos = ScreenCoord::mouse_pos(game.board.size);
        control_status.targeting = game.get_tile_at_pos(control_status.mouse_pos);
        control_status.hovering = game.get_object_at_pos(control_status.mouse_pos);
        control_status.action = update_mouse_action(&control_status);
        
        // handle events
        match &control_status.action {
            MouseAction::Dragging => {
                if control_status.dragging.is_none() {
                    // if we are not dragging already, we set the hovered object to dragging
                    if let Some(ref hovered) = control_status.hovering {
                        if hovered.props.draggable {
                            control_status.dragging = Some(hovered.clone());
                            let obj = game.get_obj_mut(hovered).unwrap();
                            obj.statuses.push(Status::Dragged);
                        }
                    }
                }
            },
            MouseAction::Drop => {
                if let Some(object) = &mut control_status.dragging {
                    let obj = game.get_obj_mut(object).unwrap();
                    obj.statuses.retain(|&x| x != Status::Dragged);
                    if let Some(target) = &control_status.targeting {
                        game.move_to(object, target.coord, curr_time, 0.25);
                    }
                }
                control_status.dragging = None;
            },
            _ => {},
        }

        match get_event() {
            Some(KbdAction::Quit) => break,
            _ => {},
        }

        rendering::render(
            &game,
            curr_time,
            &control_status,
            &render_target,
        );
        next_frame().await;
    }
}


fn get_event() -> Option<KbdAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(KbdAction::Quit);
    }
    None
}


fn update_mouse_action(status: &ControlStatus) -> MouseAction {
    if is_mouse_button_released(MouseButton::Left) {
        match &status.action {
            MouseAction::Dragging => {
                return MouseAction::Drop;
            },
            _ => {},
        };
    } else if is_mouse_button_pressed(MouseButton::Left) && status.hovering.is_some() {
        return MouseAction::Dragging;
    } else if is_mouse_button_down(MouseButton::Left) {
        return status.action.clone();
    }
    MouseAction::None
}
