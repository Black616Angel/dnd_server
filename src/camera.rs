use crate::types::*;

use macroquad::prelude::*;
pub struct Camera {
    pub position: Point,
    right_mouse: MB,
}

impl Camera {
    pub fn new() -> Self {
        let mut right_mouse = MB{ was_up: true, start: (0_f32,0_f32)};

        return Camera{position: (0_f32,0_f32), right_mouse}
    }

    pub fn x(&self) -> f32 {
        self.position.0.clone()
    }
    pub fn y(&self) -> f32 {
        self.position.1.clone()
    }

    pub fn movement(&mut self) -> Point {
        if is_mouse_button_down(MouseButton::Right){
            if self.right_mouse.was_up {
                self.right_mouse.was_up = false;
                self.right_mouse.start = mouse_position();
            } else {
                self.position.0 -= self.right_mouse.start.0 - mouse_position().0;
                self.position.1 -= self.right_mouse.start.1 - mouse_position().1;
                self.right_mouse.start = mouse_position(); 
            }
            // println!("1 Position: {},{}", right_mouse.start.0,right_mouse.start.1);
        } else if !self.right_mouse.was_up && !is_mouse_button_down(MouseButton::Right){
            self.right_mouse.was_up = true;
        }
        return self.position
    }
    
}