use serde::de::{Deserialize};

#[derive(Deserialize)]
pub struct BasicResponse{
    pub server_name:String,
    pub action:String,
    pub status:String,
    pub target: Option<String>,
    pub target_value:Option<Vec<String>>
}