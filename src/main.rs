use macroquad::prelude::*;
use hexaroni::game::{Game, Tile, HexCoord, Object, ObjectType};


fn window_conf() -> Conf {
    Conf {
        window_title: "Hexaroni".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn screen_size(board_size: usize) -> f32 {
    f32::min(
        0.33 * screen_width() / board_size as f32,
        0.5 * screen_height() / (1 + board_size) as f32,
    )
}

fn to_screen_coord(coord: &HexCoord, size: f32) -> (f32, f32) {
    let offset_x = size * (1 + coord.y) as f32;
    let offset_y = 2.0 * size;
    let x = offset_x + (2.0 * size * coord.x as f32);
    let y = offset_y + (2.0 * size * coord.y as f32);
    (x, y)
}


fn draw_tile(tile: &Tile, size: f32) {
    let (x, y) = to_screen_coord(&tile.coord, size);
    draw_hexagon(
        x,
        y,
        size,
        0.12 * size,
        true,
        Color::from_hex(0x000000),
        Color::from_hex(0x444444),
    );
}

fn draw_object(object: &Object, size: f32) {
    let color = match object.otype {
        ObjectType::Jumper => Color::from_hex(0xb04311),
        ObjectType::Dasher => Color::from_hex(0x6122c7),
        ObjectType::Wall => Color::from_hex(0x111111),
    };
    let (x, y) = to_screen_coord(&object.coord, size);
    draw_circle(x, y, 0.75 * size, color);
}


#[macroquad::main(window_conf)]
async fn main() {
    // set_fullscreen(true);
    let game = Game::new();
    loop {
        clear_background(Color::new(0.2, 0.15, 0.22, 1.0));
        let size = screen_size(game.board.size);
        for tile in game.board.tiles.iter() {
            draw_tile(tile, size);
        }
        for object in game.board.objects.iter() {
            draw_object(object, size);
        }
        
        next_frame().await
    }
}
