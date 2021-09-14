

extern crate tungstenite;
extern crate url;
extern crate chrono;
extern crate gpio;
extern crate ansi_term;

#[macro_use]
extern crate serde_json;

mod console_logger;
mod facing_data;
mod client;

use std::io::{stdin,stdout,Write};
use client::Client;


fn gather_field(output_data:&str)-> String{
    let mut data = String::new();
    print!("{}",output_data);
    stdout().flush().unwrap();
    stdin.read_line(&mut data);
}

fn main() {
    let outside_name = gather_field("server name:");
    let host = gather_field("host:");
    let port = gather_field("port:");
    let password =  gather_field("password:");
    let admin_password = gather_field("admin password:");
    let super_admin_password = gather_field("super admin password:");
    let name = gather_field("what will this tool be named on the server:");

    let client = Client::new(host,port,password,name,outside_name,admin_password,super_admin_password);
    client.begin_monitoring();
}