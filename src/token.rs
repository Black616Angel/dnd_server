use crate::types::*;

use macroquad::prelude::*;

pub enum ClickMode {
    CLICKED,
    DRAGGED,
    NONE,
}
pub struct Token {
    pub position: Vec2D,
    pub size: Vec2D,
    texture: Texture2D,
    click_counter: i32,
    pos_old: Vec2D,
}

impl Token {
    pub async fn new(position: Vec2D, texture_path: String) -> Self {
        let texture = load_texture(&texture_path).await.unwrap();
        let size: Vec2D = Vec2D::from((texture.width(), texture.height()));
        Token{ position: position.clone(), size, texture, click_counter: 0, pos_old: position.clone() }
    }
    
    pub fn in_token(&self, position: Vec2D) -> bool {
        let origin = self.position.clone() - self.size.clone() / 2_f32;
        let end = self.position.clone() + self.size.clone() / 2_f32;
        let in___ = origin <= position && end >= position;
        // println!("{}, origin: {:?}, end: {:?}, position: {:?}", in___, origin, end, position);
        // println!("origin<=position: {:?}, end>=position: {:?}",  origin<=position, end>=position);
        return in___;
    }

    pub fn movement(&mut self, direction: Vec2D) {
        self.position += direction;
    }

    pub fn final_move(&mut self) {
        self.position += Vec2D::square((SQUARE_SIZE / 2) as f32) - (self.position.clone()) % SQUARE_SIZE as f32;
        self.pos_old = self.position.clone();
        // TODO: show distance
    }

    pub fn draw(&self, offset: &Vec2D) {
        let pos = self.position.clone() + offset - Vec2D::from((self.texture.width(), self.texture.height())) / 2_f32;
        draw_texture(self.texture, pos.x, pos.y, Color::from_rgba(255, 255, 255, 255));
    }

    pub fn click(&mut self) {
    }

    pub fn clicked(&mut self, clicked: bool) -> ClickMode {
        if clicked {
            self.click_counter += 1;
            if self.click_counter >= 10 {
                return ClickMode::DRAGGED;
            }
        } else {
            if self.click_counter < 10 {
                self.click_counter = 0;
                return ClickMode::CLICKED;
            } else {
                self.click_counter = 0;
                return ClickMode::NONE;
            }
        }
        return ClickMode::NONE;
    }
}

pub struct Tokenlist {
    list: Vec<Token>,
    left_mouse: MB,
    active_token_idx: Option<i32>,
}

impl Tokenlist {

    pub fn new(tokens: Option<Vec<Token>>) -> Self {
        let left_mouse = MB{ was_up: true, start: Vec2D::empty()};
        if let Some(tokens) = tokens {
            return Self{ list: tokens, left_mouse, active_token_idx: None };
        } else {
            return Self{ list: Vec::new(), left_mouse, active_token_idx: None };
        }
    }

    pub fn add(&mut self, mut token: Token) {
        token.final_move();
        self.list.push(token);
    }

    pub fn click(&mut self, position: Vec2D) {
        // println!("x: {}, y: {}", position.0 - mouse_position().0, position.1 - mouse_position().1);

        if is_mouse_button_down(MouseButton::Left){
            if self.left_mouse.was_up {
                self.left_mouse.was_up = false;
                self.left_mouse.start = Vec2D::from(mouse_position());
                let mut idx = 0;
                for token in &mut self.list {
                    if token.in_token(position.clone()) {
                        self.active_token_idx = Some(idx);
                        token.clicked(true);
                        break;
                    }
                    idx += 1;
                }
            } else if self.active_token_idx != None {
                let drag = Vec2D::empty() - self.left_mouse.start.clone() + mouse_position();
                // println!("drag: {:?}", drag);
                let token = self.list.get_mut(self.active_token_idx.unwrap() as usize).unwrap();
                token.movement(drag);
                self.left_mouse.start = Vec2D::from(mouse_position());
            }
        } else if !self.left_mouse.was_up && !is_mouse_button_down(MouseButton::Left){
            self.left_mouse.was_up = true;
            if let Some(index) = self.active_token_idx {
                let token = self.list.get_mut(index as usize).unwrap();
                match token.clicked(false){
                    ClickMode::CLICKED => token.click(),
                    _ => {},
                }
                token.final_move();
                self.active_token_idx = None;
            }
            self.active_token_idx = None;
        }
    }

    pub fn draw_all(&self, offset: &Vec2D) {
        for token in &self.list {
            token.draw(&offset);
        }
    }
}