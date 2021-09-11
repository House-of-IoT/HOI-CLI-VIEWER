pub struct Facing{
    different_bots:i32,
    non_bots:i32,
    all_devices:i32,
    config:Config,
    contacts:Vec<String>,
    connection_string:String,
    banned_ips : Vec<String> 
}

pub struct Config{
    deactivating:bool,
    activating:bool,
    disconnecting:bool,
    viewing:bool
}