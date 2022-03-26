use crate::types::*;

use macroquad::prelude::*;
pub struct Camera {
    pub position: Vec2D,
    right_mouse: MB,
}

impl Camera {
    pub fn new() -> Self {
        let right_mouse = MB{ was_up: true, start: Vec2D::empty()};

        return Camera{position: Vec2D::empty(), right_mouse}
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }
    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn movement(&mut self) -> Vec2D {
        if is_mouse_button_down(MouseButton::Right){
            if self.right_mouse.was_up {
                self.right_mouse.was_up = false;
                self.right_mouse.start = Vec2D::from(mouse_position());
            } else {
                self.position -= self.right_mouse.start.clone() - mouse_position();
                self.right_mouse.start = Vec2D::from(mouse_position()); 
            }
            // println!("1 Position: {},{}", right_mouse.start.0,right_mouse.start.1);
        } else if !self.right_mouse.was_up && !is_mouse_button_down(MouseButton::Right){
            self.right_mouse.was_up = true;
        }
        return self.position.clone()
    }
    
}