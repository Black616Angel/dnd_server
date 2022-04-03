use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneJson {
    /// scene name: only important for overview
    pub name: String,
    /// scene height in squares
    pub height: i32,
    /// scene width in squares
    pub width: i32,
    /// square size in pixels
    /// this ist also used to resize the tokens
    pub square_size: i32,
    /// list of tokens
    pub tokens: Vec<SceneJsonToken>,
    /// texture of the background
    /// this will be resized to fit the scene size (squares * square size)
    pub texture_background: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneJsonToken {
    /// token name
    pub name: String,
    /// token description
    pub description: Option<String>,
    /// token texture
    pub texture_path: String,
    /// list of the token stats
    pub stats: Option<Vec<SceneJsonStat>>,
    /// token height in squares
    pub height: i32,
    /// token width in squares
    pub width: i32,
    /// x-postition on squares (not pixels)
    pub position_x: i32,
    /// y-postition on squares (not pixels)
    pub position_y: i32,
    /// action on single left click
    pub click_action: Option<ClickAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneJsonStat {
    /// stat name
    pub name: String,
    /// stat value
    pub value: Value,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "action")]
pub enum ClickAction {
    /// scene changes after click on token
    SceneChange(String),
    /// show token stats
    ShowStats(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastSceneJson {
    pub name: String,
}