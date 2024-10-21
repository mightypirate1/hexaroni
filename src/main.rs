use std::time::Instant;
use macroquad::prelude::*;
use hexaroni::game::Game;
use hexaroni::renderer::draw;

enum Event {
    Exit,
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
    // set_fullscreen(true);
    let start_time = Instant::now();
    let game = Game::new();
    loop {
        let curr_time = start_time.elapsed().as_secs_f32();

        match get_event() {
            Some(Event::Exit) => break,
            _ => {}
        }

        render(&game, curr_time);
        next_frame().await
    }
}


fn get_event() -> Option<Event> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(Event::Exit);
    }
    None
}

fn render(game: &Game, time: f32) {
    draw::background();
    for tile in game.board.tiles.iter() {
        draw::tile(tile, time);
    }
    for object in game.board.objects.iter() {
        draw::object(object, time);
    }
}