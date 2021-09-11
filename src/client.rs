use console_logger::ConsoleLogger;
use facing_data::Facing;
use facing_data::Config;
use serde_json;

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
        super_pw : String,
        pw : String)->Self{

        Self{
            host:host_data.to_string(),
            port:port_data,
            password:pw,
            super_admin_password:super_pw,
            admin_password:admin_pw
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

            if msg != ""{

            }
            else{
                self.logger.log_error_encounter();
                break;
            }
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
        let data =  self.gather_message(socket);
        if data != ""{
            //parse json and continue accordingly if auth is needed
            let mut data = serde_json::from_str(data);
            if data.is_ok(){
                data = data.unwrap();

            }
            else{
                println!("Issue parsing json");
            }
        }
        else{
            //log issue gathering 
        }
    }   

    // checks to see if we need admin authentication or not
    fn check_and_handle_two_way_request_response(&mut self, socket:&mut WebSocket<AutoStream>,response:Value)->String{
        if response["status"] == "needs-admin-auth"{
            if response["action"] == "editing" {
                self.send_message(self.super_admin_password);
            }
            else{
                self.send_message(self.admin_password);
            }
        }
        else if response["status"] == "success"{

        }
        // timeout or failure
        else{
            
        }
    }

    fn gather_deactivated_bots(&mut self, socket:&mut WebSocket<AutoStream>){
        // use execute two way request
    }
}