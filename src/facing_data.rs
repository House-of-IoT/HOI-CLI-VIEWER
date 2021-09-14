pub struct Facing{
    pub different_bots:i32,
    pub non_bots:i32,
    pub all_devices:i32,
    pub config:Config,
    pub contacts:Vec<String>,
    pub connection_string:String,
    pub banned_ips : Vec<String> 
}

pub struct Config{
    pub deactivating:String,
    pub activating:String,
    pub disconnecting:String,
    pub viewing:String
}