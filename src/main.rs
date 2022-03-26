mod camera;
mod types;
mod token;

use macroquad::prelude::*;

use types::*;
use camera::Camera as Cam;
use token::*;

#[macroquad::main("PnP")]
async fn main() {
    let mut cam = Cam::new();
    let mut tokens = Tokenlist::new(None);
    tokens.add(Token::new(Vec2D::empty(), "assets/rustacean_happy.png".to_string()).await);

    loop {
        // Move camera, get new offset
        cam.movement();
        let local_mouse_pos = Vec2D::from((mouse_position().0 - cam.x(), mouse_position().1 - cam.y()));

        draw_rectangle(0_f32, 0_f32, screen_width(), screen_height(), WHITE);
        draw_squares(SQUARES_X, SQUARES_Y, cam.x(), cam.y());
        tokens.click( local_mouse_pos);
        tokens.draw_all(&cam.position);
        next_frame().await
    }
}

fn draw_squares(x: i16, y: i16, offset_x: f32, offset_y: f32) {
    // let sq_size = (screen_height() - offset_y * 2.) / x as f32;
    let sq_size = SQUARE_SIZE as f32;
    for i in 0..x+1 {
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            (sq_size * x as f32) + offset_x,
            offset_y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 0..y+1 {
        draw_line(
            offset_x + sq_size * i as f32,
            offset_y,
            offset_x + sq_size * i as f32,
            (sq_size * y as f32) + offset_y,
            2.,
            LIGHTGRAY,
        );
    }
}