use std::time::{Duration, Instant};
use macroquad::prelude::*;
use hexaroni::game::{Game, Object, ObjectType};
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
    let pixelization = 1.0;
    let render_target = render_target(
        (screen_width() / pixelization) as u32,
        (screen_height() / pixelization) as u32,
    );
    render_target.texture.set_filter(FilterMode::Nearest);

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
        control_status.targeting = get_tile_at_pos(control_status.mouse_pos, &game);
        control_status.hovering = get_object_at_pos(control_status.mouse_pos, &game);
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

fn get_object_at_pos(pos: ScreenCoord, game: &Game) -> Option<Object> {
    game.board.objects
        .iter()
        .find(|o| {
            match o.otype {
                ObjectType::Tile => false,
                _ => is_close(pos, o.pos),
            }
        })
        .cloned()
}

fn get_tile_at_pos(pos: ScreenCoord, game: &Game) -> Option<Object> {   
    game.board.objects
        .iter()
        .find(|o| {
            match o.otype {
                ObjectType::Tile => is_close(pos, o.pos),
                _ => false,
            }
        })
        .cloned()
}

fn is_close(me: ScreenCoord, other: ScreenCoord) -> bool {
    let delta = me.sub(&other);
    let distance_sq = delta.x.powi(2) + delta.y.powi(2);
    distance_sq < 0.75 * me.screen_size.powi(2)
}
