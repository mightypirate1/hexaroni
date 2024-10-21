use std::time::Instant;
use macroquad::prelude::*;
use hexaroni::game::Game;
use hexaroni::renderer::draw;

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
    let time = Instant::now();
    let game = Game::new();
    loop {
        draw::background();
        for tile in game.board.tiles.iter() {
            draw::tile(tile, time.elapsed().as_secs_f32());
        }
        for object in game.board.objects.iter() {
            draw::object(object, time.elapsed().as_secs_f32());
        }
        
        next_frame().await
    }
}
