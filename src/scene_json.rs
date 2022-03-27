use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SceneJson {
    pub name: String,
    pub height: i32,
    pub width: i32,
    pub square_size: i32,
    pub tokens: Vec<SceneJsonToken>,
    pub texture_background: String,
}

#[derive(Deserialize)]
pub struct SceneJsonToken {
    pub name: String,
    pub description: String,
    pub texture_path: String,
    pub stats: Vec<SceneJsonStat>,
    pub height: i32,
    pub width: i32,
    pub position_x: i32,
    pub position_y: i32,
}

#[derive(Deserialize)]
pub struct SceneJsonStat {
    pub name: String,
    pub value: Value,
}