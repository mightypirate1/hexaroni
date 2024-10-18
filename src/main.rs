use macroquad::prelude::*;
use hexaroni::game::{Game, Block, HexCoord, Object, ObjectType};


fn window_conf() -> Conf {
    Conf {
        window_title: "Hexaroni".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}


fn draw_block(block: &Block) {
    let size = f32::min(
        0.33 * screen_width() / (block.coord.board_size) as f32,
        0.5 * screen_height() / (1 + block.coord.board_size) as f32,
    );
    let (x, y, size) = to_screen_coord(&block.coord, size);
    draw_hexagon(
        x,
        y,
        size,
        0.12 * size,
        true,
        Color::from_hex(0x000000),
        Color::from_hex(0x444444),
    );
    if let Some(object) = &block.object {
        draw_object(&object, x, y, size);
    }
}

fn draw_object(object: &Object, x: f32, y: f32, size: f32) {
    let color = match object.otype {
        ObjectType::Jumper => Color::from_hex(0xb04311),
        ObjectType::Dasher => Color::from_hex(0x6122c7),
        ObjectType::Wall => Color::from_hex(0x111111),
    };
    draw_circle(x, y, 0.75 * size, color);
}

fn to_screen_coord(coord: &HexCoord, size: f32) -> (f32, f32, f32) {
    let offset_x = size * (1 + coord.y) as f32;
    let offset_y = 2.0 * size;
    let x = offset_x + (2.0 * size * coord.x as f32);
    let y = offset_y + (2.0 * size * coord.y as f32);
    (x, y, size)
}


#[macroquad::main(window_conf)]
async fn main() {
    set_fullscreen(true);
    let game = Game::new();
    loop {
        clear_background(Color::new(0.2, 0.15, 0.22, 1.0));
        for block in game.board.blocks.iter() {
            draw_block(block);
        }
        
        next_frame().await
    }
}
