pub mod camera;
pub mod types;

use macroquad::prelude::*;

use crate::types::*;
use crate::camera::Camera as Cam;

const SQUARES_X: i16 = 4;
const SQUARES_Y: i16 = 4;


#[macroquad::main("DnD")]
async fn main() {
    let mut cam = Cam::new();

    loop {
        cam.movement();
        draw_rectangle(0_f32, 0_f32, screen_width(), screen_height(), WHITE);
        draw_squares(SQUARES_X, SQUARES_Y, cam.x(), cam.y());

        // draw_text(
        //     format!("Position: {},{}", right.start.0,right.start.1).as_str(),
        //     10.,
        //     10.,
        //     20.,
        //     BLACK,
        // );
        next_frame().await
    }
}

fn draw_squares(x: i16, y: i16, offset_x: f32, offset_y: f32) {
    // let sq_size = (screen_height() - offset_y * 2.) / x as f32;
    let sq_size = 70_f32;
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