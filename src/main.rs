use std::time::Instant;
use macroquad::prelude::*;
use hexaroni::game::{Game, Object};
use hexaroni::ui::rendering;
use hexaroni::geometry::ScreenCoord;


// enum Event {
//     Exit,
// }

#[derive(Debug, Clone)]
enum MouseAction {
    None,
    Dragging,
    Drop,
}

#[derive(Debug, Clone)]
struct ControlStatus {
    mouse_pos: ScreenCoord,
    action: MouseAction,
    hovering: Option<Object>,
    dragging: Option<Object>,
    targeting: Option<Object>,
}

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
        control_status.mouse_pos = get_mouse_pos(&game);
        control_status.targeting = game.get_tile_at_pos(control_status.mouse_pos);
        control_status.hovering = game.get_object_at_pos(control_status.mouse_pos);
        control_status.action = update_mouse_action(&control_status);

        // handle events
        match &control_status.action {
            MouseAction::Dragging => {
                if let Some(ref dragged) = control_status.dragging {
                    game.set_pos(&dragged, control_status.mouse_pos);
                } else if let Some(ref hovered) = control_status.hovering {
                    if hovered.props.draggable {
                        control_status.dragging = Some(hovered.clone());
                        game.set_pos(&hovered, control_status.mouse_pos);
                    }
                }
            },
            MouseAction::Drop => {
                if let Some(object) = &mut control_status.dragging {
                    if let Some(target) = &control_status.targeting {
                        game.move_to(object, target.coord, curr_time, 0.1);
                    } else {
                        game.move_to(object, object.coord, curr_time, 0.1);
                    }
                }
                control_status.dragging = None;
            },
            _ => {},
        }

        rendering::render(&game, curr_time, &render_target);
        next_frame().await;
    }
}

fn get_mouse_pos(game: &Game) -> ScreenCoord {
    let (x, y) = mouse_position();
    ScreenCoord::new(x, y, game.board.size)
}

// fn get_event() -> Option<Event> {
//     if is_key_pressed(KeyCode::Escape) {
//         return Some(Event::Exit);
//     }
//     None
// }

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
