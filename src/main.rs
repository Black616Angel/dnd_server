mod camera;
mod types;
mod token;
mod scene;
mod scene_json;

use macroquad::prelude::*;

use crate::scene::*;

#[macroquad::main("PnP")]
async fn main() {
    let mut scene = Scene::new_from_file("assets/Test_Scene.json".to_string()).await.unwrap();
    loop {
        // Move camera, get new offset
        scene.draw();
        next_frame().await
    }
}