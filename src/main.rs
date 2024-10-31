use hexaroni::engine::{statuses::Status, Game};
use hexaroni::geometry::ScreenCoord;
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
    let render_scale = 4.0;
    let mut renderer = Renderer::new(render_scale).unwrap();
    let mut curr_window_size = screen_size();

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
        if curr_window_size != screen_size() {
            match Renderer::new(render_scale) {
                Ok(r) => renderer = r,
                Err(msg) => println!("{}", msg),
            };
            curr_window_size = screen_size();
        }

        // read mouse status
        control_status.mouse_pos = ScreenCoord::mouse_pos(game.board.size);
        control_status.hovering = game.get_object_at_pos(control_status.mouse_pos);
        control_status.action = update_mouse_action(&control_status);
        control_status.targeting = match &control_status.hovering {
            None => game.get_tile_at_pos(control_status.mouse_pos),
            Some(object) => {
                let targetable_player = if control_status.action == MouseAction::None {
                    game.board.current_player
                } else {
                    game.board.current_player.opponent()
                };
                if object.owned_by(&targetable_player) {
                    game.get_tile_at_pos(control_status.mouse_pos)
                } else {
                    None
                }
            }
        };

        // handle events
        match &control_status.action {
            MouseAction::Dragging => {
                if control_status.dragging.is_none() {
                    // if we are not dragging already, we set the hovered object to dragging
                    if let Some(ref hovered) = control_status.hovering {
                        if hovered.props.draggable
                            && hovered.owned_by(&game.board.current_player)
                            && !hovered.props.dead
                        {
                            control_status.dragging = Some(Drag::create(hovered, &mut game));
                        }
                    }
                }
            }
            MouseAction::Drop => {
                if let Some(drag) = &mut control_status.dragging {
                    let obj = game.get_obj_mut(&drag.object).unwrap();
                    obj.remove_status(Status::Dragged);
                    if let Some(target_tile) = &control_status.targeting {
                        if let Some(r#move) = drag.get_move_to(&target_tile.coord) {
                            game.apply_move(r#move, curr_time, 0.25);
                        }
                    }
                }
                control_status.dragging = None;
            }
            _ => {}
        }

        match get_event() {
            Some(KbdAction::Quit) => break,
            Some(KbdAction::ReloadShader) => {
                match Renderer::new(render_scale) {
                    Ok(r) => renderer = r,
                    Err(msg) => println!("{}", msg),
                };
            }
            _ => {}
        }

        renderer.render(&game, &control_status, curr_time);
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
    None
}

fn update_mouse_action(status: &ControlStatus) -> MouseAction {
    if is_mouse_button_released(MouseButton::Left) {
        if status.action == MouseAction::Dragging {
            return MouseAction::Drop;
        }
    } else if is_mouse_button_pressed(MouseButton::Left) && status.hovering.is_some() {
        return MouseAction::Dragging;
    } else if is_mouse_button_down(MouseButton::Left) {
        return status.action.clone();
    }
    MouseAction::None
}
