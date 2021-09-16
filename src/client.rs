use crate::console_logger::ConsoleLogger;

use crate::facing_data::Config;
use crate::facing_data::Facing;
use serde_json::{Value};
use std::{thread, time};
use tungstenite::{connect, Message,WebSocket};
use tungstenite::client::AutoStream;
use url::Url;
use std::env;
use serde_json::Map;
use std::convert::TryInto;

pub struct Client{
    host:String,
    port:String,
    password:String,
    admin_password:String,
    super_admin_password:String,
    name:String,
    server_name:String,
    logger:ConsoleLogger,
    device_type :String
}

impl Client{

    pub fn new(host_data:String,
        port_data:String,
        pass:String, 
        device_name:String,
        outside_server_name:String,
        admin_pw : String,
        super_pw : String)->Self{

        Self{
            host:host_data.to_string(),
            port:port_data,
            password:pass,
            super_admin_password:super_pw,
            admin_password:admin_pw,
            name:device_name,
            device_type:"non-bot".to_owned(),
            server_name:outside_server_name,
            logger: ConsoleLogger::new()
        }
    }

    fn check_auth_response(&mut self,  socket:&mut WebSocket<AutoStream>)-> bool{
        let msg_result = socket.read_message();

        if msg_result.is_ok(){
            let msg = msg_result.unwrap().into_text().unwrap();
            if msg == "success"{
                self.logger.log_basic_row("Successfully Authenticated!!\n","green");
                return true;
            }
            else{
                self.logger.log_failed_auth();
                return false;
            }
        }
        else{
            self.logger.log_failed_auth();
            return false;
        }
    }

    pub fn authenticate(&mut self, socket:&mut WebSocket<AutoStream>)->bool{
        let send_password_result = socket.write_message(Message::Text(self.password.to_owned()));

        //send password
        if send_password_result.is_ok(){
            let send_name_and_type_result =
                socket.write_message(Message::Text(self.name_and_type()));
            //send name and type    
            if send_name_and_type_result.is_ok(){
                let send_server_name_result = 
                    socket.write_message(Message::Text(self.server_name.to_owned()));
                    // send server name
                if send_server_name_result.is_ok(){
                    return self.check_auth_response(socket);
                }
            }
        }
        return false;
    }

    pub fn begin_monitoring(&mut self){
        self.logger.log_welcome();
        let url = format!("ws://{}:{}",self.host,self.port);
        let attempt = connect(Url::parse(&url).unwrap());

        if attempt.is_ok(){
            let (mut socket, response) = attempt.unwrap();
            //if we successfully authenticated
            if self.authenticate(&mut socket) == true{
                self.enter_main_loop(&mut socket);
            }
            else{
                self.logger.log_failed_auth();
                self.logger.log_error_encounter();
            }
        }
        else{
            self.logger.log_error_encounter();
        }
    }

    //keep listening for server requests and route the requests
    fn enter_main_loop(&mut self,socket:&mut WebSocket<AutoStream>){
        loop {
            let data :Facing = self.gather_all_facing_data(socket);
            self.logger.log_interval_data(data,self.server_name.clone());
            //run on a ten second interval
            let milliseconds_to_sleep = time::Duration::from_millis(10000);
            thread::sleep(milliseconds_to_sleep);
        }
    }

    fn gather_message(&mut self,socket:&mut WebSocket<AutoStream>)->String{
        let msg_result = socket.read_message();
        if msg_result.is_ok(){
            let msg = msg_result.unwrap().into_text().unwrap();
            return msg;
        }
        else{
            return "".to_owned();
        }
    } 

    fn send_message(&mut self, socket:&mut WebSocket<AutoStream>,data:String)-> bool{
        let result = socket.write_message(Message::Text(data));
        if result.is_ok(){
            println!("issue sending data via socket");
            return true;
        }
        else{
            return false;
        }
    }

    fn execute_two_way_request(&mut self, socket:&mut WebSocket<AutoStream>,message:String)->String{
        self.send_message(socket,message);
        let mut data =  self.gather_message(socket);
        if data != ""{
            //parse json and continue accordingly if auth is needed
            let data_result = serde_json::from_str(&data);
            if data_result.is_ok(){
                data = data_result.unwrap();
                return self.check_and_handle_two_way_request_response(socket,serde_json::Value::String(data));
            }
            else{
                println!("Issue parsing json");
                return String::new();
            }
        }
        else{
            //log issue gathering 
            return String::new();
        }
    }   

    // checks to see if we need admin authentication.
    //if admin auth is needed, handles admin auth and return the result of the post auth response
    fn check_and_handle_two_way_request_response(&mut self, socket:&mut WebSocket<AutoStream>,response:Value)->String{
        if response["status"] == "needs-admin-auth"{
            let mut send_result;
            if response["action"] == "editing" {
                send_result = self.send_message(socket,self.super_admin_password.clone());
            }
            else{
                send_result = self.send_message(socket,self.admin_password.clone());
            }

            //check the send result and handle second response recursively
            if send_result == true{
                let second_response = self.gather_message(socket);
                return self.check_and_handle_two_way_request_response(socket,serde_json::Value::String(second_response));
            }
            else{
                return String::new();
            }
        }
        else if response["status"] == "success"{
            return response["target_value"].to_string();
        }
        // timeout or failure
        else{
            return String::new();
        }
    }

    fn gather_all_facing_data(&mut self,  socket:&mut WebSocket<AutoStream>)-> Facing{
        let deactivated_bots = self.execute_two_way_request(socket,"servers_deactivated_bots".to_string());
        let all_devices = self.execute_two_way_request(socket,"servers_devices".to_string());
        let banned_ips = self.execute_two_way_request(socket,"servers_banned_ips".to_string());
        let config = self.execute_two_way_request(socket,"server_config".to_string());
        let contacts = self.execute_two_way_request(socket,"contact_list".to_string());

        let mut deactivated_bots_len:i32 = -1;
        let mut all_devices_len:i32 = -1;
        let mut banned_ips_len:i32 = -1;
        let mut contact_len:i32 = -1;
        let mut config_holder;
        let empty_config = Config{
            deactivating:String::new(),
            activating:String::new(),
            viewing:String::new(),
            disconnecting:String::new()
        };

        if deactivated_bots != ""{
            deactivated_bots_len = self.extract_json_len(&deactivated_bots);
        }
        if all_devices != ""{
            all_devices_len = self.extract_json_len( &all_devices);
        }
        if banned_ips != ""{
            banned_ips_len = self.extract_json_len_vec(banned_ips);
        }
 
        if contacts != ""{
            contact_len = self.extract_json_len(&contacts);
        }
        if config != ""{
            let data = self.gather_config_from_json(contacts);
            if data.is_some(){
                config_holder = data.unwrap();
            }
            else{
                config_holder = empty_config;
            }
        }
        else{
            config_holder = empty_config;
        }
    
        //need to implement the logic for the values with -1
        return Facing{
            different_bots: -1,
            non_bots : -1,
            all_devices: all_devices_len,
            config : config_holder,
            contacts:contact_len,
            connection_string: format!("{}:{}",self.host,self.port),
            banned_ips:banned_ips_len
        }
    }

    //intended to be used with map
    fn extract_json_len(&mut self, data:&String)-> i32{
        let json_data : std::result::Result<serde_json::Map<String, Value>, serde_json::Error> = serde_json::from_str(&data);
        if json_data.is_ok(){
            let len : i32 = json_data.unwrap().len().try_into().unwrap();
            return len;
        }
        else{
            return -1;
        }
    }

    fn extract_json_len_vec(&mut self, data:String)-> i32{
        let json_data: std::result::Result<Vec<String>, serde_json::Error>  = serde_json::from_str(&data);
        if json_data.is_ok(){
            let json_test  = json_data.unwrap();
            let len : i32= json_test.len().try_into().unwrap();
            return len;
        }
        else{
            return -1;
        }
    }

    fn gather_config_from_json(&mut self, data:String)->Option<Config>{
        let json_data = serde_json::from_str(&data);
        if json_data.is_ok(){
            let parsed_data: Config = json_data.unwrap();
            return Some(parsed_data);
        }
        else{
            return None;
        }
    }

    fn name_and_type(&mut self)-> String{
        let name_and_type = json!({
            "name":&self.name,
            "type":&self.device_type
        });
        return name_and_type.to_string();
    }
}