use crate::types::*;
use crate::token::*;
use crate::camera::Camera as Cam;
use crate::scene_json::*;

use std::fs;
use std::error::Error;
use macroquad::prelude::*;

pub struct Scene {
    name: String,
    pub height: i32,
    pub width: i32,
    cam: Cam,
    tokens: Tokenlist,
    square_size: f32,
}

impl Scene {
    pub async fn new_from_file(filename: String) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        return Self::new_from_string(contents).await
    }

    pub async fn new_from_string(data: String) -> Result<Self, Box<dyn Error>> {
        let json: SceneJson = serde_json::from_str(&data)?;
        return Ok(Self::new_from_json(json).await)
    }

    pub async fn new_from_json(json: SceneJson) -> Self {
        let name = json.name;
        let height = json.height;
        let width = json.width;
        let square_size = json.square_size as f32;
        let mut tokens: Tokenlist = Tokenlist::new(None);
        for token in json.tokens {
            tokens.add(Token::new_from_json(token, square_size).await, square_size)
        }
        Self{name, height, width, cam: Cam::new(), tokens, square_size}
    }

    pub fn draw(&mut self) {
        self.cam.movement();
        let local_mouse_pos = Vec2D::from((mouse_position().0 - self.cam.x(), mouse_position().1 - self.cam.y()));
        self.scrolling();
        draw_rectangle(0_f32, 0_f32, screen_width(), screen_height(), WHITE);
        self.draw_squares(self.width, self.height, self.cam.x(), self.cam.y());
        self.tokens.click( local_mouse_pos, self.square_size);
        self.tokens.draw_all(&self.cam.position, self.square_size);
    }

    fn scrolling(&mut self) {
        let mut changed = false;
        if is_key_pressed(KeyCode::KpAdd) {
            self.square_size += 10_f32;
            changed = true;
        }
        if is_key_pressed(KeyCode::KpSubtract) {
            self.square_size -= 10_f32;
            changed = true;
        }
        if changed {
            self.tokens.final_move(self.square_size);
        }
    }

    fn draw_squares(&self, x: i32, y: i32, offset_x: f32, offset_y: f32) {
        // let sq_size = (screen_height() - offset_y * 2.) / x as f32;
        let sq_size = self.square_size;
        for i in 0..x+1 {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                (sq_size * y as f32) + offset_x,
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
                (sq_size * x as f32) + offset_y,
                2.,
                LIGHTGRAY,
            );
        }
    }
}