use serde::de::{Deserialize};

pub struct Facing{
    pub different_bots:i32,
    pub non_bots:i32,
    pub all_devices:i32,
    pub config:Config,
    pub contacts:i32,
    pub connection_string:String,
    pub banned_ips : i32 
}

#[derive(Deserialize)]
pub struct Config{
    pub deactivating:String,
    pub activating:String,
    pub disconnecting:String,
    pub viewing:String
}


