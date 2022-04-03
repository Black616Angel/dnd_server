mod camera;
mod types;
mod token;
mod scene;
mod scene_json;
mod ui;
mod game_picker;

use macroquad::prelude::*;
use scene_json::ClickAction;
use scene_json::SceneJson;
use scene_json::SceneJsonToken;
use std::error::Error;

use crate::scene::*;
use crate::ui::*;
use crate::types::*;
use crate::game_picker::*;

#[macroquad::main("PnP")]
async fn main() {

    let gp = GamePicker::new("files".to_string());
    let mut ui = UIList::new();
    // ui.add_button(Button::new("text".to_string(), Vec2D::new(10_f32, 20_f32),
    // Vec2D::new(200_f32, 100_f32), Color { r: 200_f32, g: 200_f32, b: 200_f32, a: 255_f32 }, ||{println!("klappt");}));

    info!("Started");
    // let mut scene = Scene::new_from_file("files/games/testgame/Test_Scene.json".to_string(), None).await.unwrap();
    let mut scene = gp.get_scene().await.unwrap();
    let mut next_scene: Option<String> = None;
    loop {
        if let Some(action) =  scene.click() {
            match action {
                ClickAction::SceneChange(scene) => {
                    next_scene = Some(scene);
                },
                _ => {}
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            ui.click();
        }

        scene.draw();
        ui.draw();
        // macroquad::ui::root_ui().label(None, "hello megaui");
        // if macroquad::ui::root_ui().button(None, "Push me") {
        //     println!("pushed");
        // }
        next_frame().await;
        if let Some(next_scene_name) = next_scene {
            next_scene = None;
            if scene.name == "Game Picker" {
                let mut split = next_scene_name.split("/");
                let folder = gp.root_folder.clone() + "/games/" + split.next().unwrap();
                let filename = folder.clone() + "/" + split.next().unwrap();
                info!("{}", filename);
                if let Ok(new) = Scene::new_from_file(filename, Some(folder)).await {
                    info!("{}", scene.name);
                    scene = new;
                };
            } else {

            }
        }
    }
}