use crate::console_logger::ConsoleLogger;

use crate::facing_data::Config;
use crate::facing_data::Facing;
use serde_json;
use std::{thread, time};
use tungstenite::{connect, Message,WebSocket};
use tungstenite::client::AutoStream;
use url::Url;
use std::env;

pub struct Client{
    host:String,
    port:String,
    password:String,
    admin_password:String,
    super_admin_password:String,
    name:String,
    server_name:String,
    logger:ConsoleLogger
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
            device_type:type_of_bot,
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
            self.console_logger.log_interval_data(data,self.outside_server_name);
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
            data = serde_json::from_str(data);
            if data.is_ok(){
                data = data.unwrap();
                return self.check_and_handle_two_way_request_response(socket,data);
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
            let mut send_result = String::new();
            if response["action"] == "editing" {
                send_result = self.send_message(self.super_admin_password);
            }
            else{
                send_result = self.send_message(self.admin_password);
            }

            //check the send result and handle second response recursively
            if send_result == true{
                let second_response = self.gather_message(socket);
                return self.check_and_handle_two_way_request_response(socket,second_response);
            }
            else{
                return String::new();
            }
        }
        else if response["status"] == "success"{
            return response["target_value"];
        }
        // timeout or failure
        else{
            return String::new();
        }
    }

    fn gather_all_facing_data(&mut self,  socket:&mut WebSocket<AutoStream>)-> Facing{
        let deactivated_bots = self.execute_two_way_request(socket,"servers_deactivated_bots");
        let all_devices = self.execute_two_way_request(socket,"servers_devices");
        let banned_ips = self.execute_two_way_request(socket,"servers_banned_ips");
        let config = self.execute_two_way_request(socket,"server_config");
        let contacts = self.execute_two_way_request(socket,"contact_list");

        let mut deactivated_bots_len:i32 = -1;
        let mut all_devices_len:i32 = -1;
        let mut banned_ips_len:i32 = -1;
        let mut contact_len:i32 = -1;
        let mut config_holder;

        if deactivated_bots != ""{
            deactivated_bots_len = self.extract_json_len(deactivated_bots);
        }
        if all_devices != ""{
            all_devices_len = self.extract_json_len(all_devices);
        }
        if banned_ips != ""{
            banned_ips_len = self.extract_json_len(banned_ips);
        }
 
        if contacts != ""{
            contact_len = self.extract_json_len(contacts);
        }
        if config != ""{
            let data = self.gather_config_from_json(contacts);
            if data.is_some(){
                config_holder = data.unwrap();
            }
        }
        //need to implement the logic for the values with -1
        return Facing{
            different_bots: -1,
            non_bots : -1,
            all_devices: all_devices_len,
            config : config_holder,
            contacts:contact_len,
            connection_string: format!("{}:{}",self.host_data,self.port_data),
            banned_ips:banned_ips_len
        }
    }

    //intended to be used with object<map> and arrays
    fn extract_json_len(&mut self, data:String)-> i32{
        let json_data = serde_json::from_str(data);
        if json_data.is_ok(){
            let len : i32 = json_data.unwrap().len();
            return len;
        }
        else{
            return -1;
        }
    }

    fn gather_config_from_json(&mut self, data:String)->Option<Config>{
        let json_data = serde_json::from_str(data);
        if json_data.is_ok(){
            let parsed_data = json_data.unwrap();
            let config = Config{
                deactivating:parsed_data["deactivating"],
                activating:parsed_data["activating"],
                disconnecting:parsed_data["disconnecting"],
                viewing:parsed_data["viewing"]
            };
            return config;
        }
        else{
            return None;
        }
    }
}