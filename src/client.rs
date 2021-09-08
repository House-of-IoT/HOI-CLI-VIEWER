pub struct Client{
    host:String,
    port:String,
    password:String,
    admin_password:String,
    super_admin_password:String,
    name:String,
    server_name:String,
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
            let msg_result = socket.read_message();
            if msg_result.is_ok(){
                let msg = msg_result.unwrap().into_text().unwrap();
                self.route_message(msg,socket,motion_last_sensed,number_of_times_sensed);
                
            }
            else{
                self.logger.log_error_encounter();
                *encountered_error = true;
                break;
            }
        }
    }

    fn gather_message(&mut self)->String{
        let msg_result = socket.read_message();
        if msg_result.is_ok(){
            let msg = msg_result.unwrap().into_text().unwrap();
            return msg;
        }
    } 
}