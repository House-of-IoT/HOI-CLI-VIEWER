use ansi_term::Colour;
use crate::facing_data::Facing;

pub struct ConsoleLogger{
    row_number:i16
}

impl ConsoleLogger{

    pub fn new() -> Self{
        Self{row_number:0}
    }

    pub fn log_basic_row(&mut self, data:&str, color:&str){
        if color == "green"{
            println!("{} - {}",self.row_number,Colour::Green.paint(data));
            self.row_number += 1;
        }
        else{
            println!("{} - {}",self.row_number,Colour::Red.paint(data));
            self.row_number += 1;
        }
    }

    pub fn log_info(&mut self, data:&str){
        println!("{}:{}", Colour::Yellow.paint("[INFO]"),data);
    }

    pub fn log_failed_auth(&mut self){
        self.log_basic_row("Failed To Authenticate!!!","red");
        self.log_info("Is your password correct?\n");
    }

    pub fn log_error_encounter(&mut self){
        self.log_basic_row("Encountered error with authentication or networking. Trying to reconnect...","red");
        self.log_info("This will keep happening until a connection to the server is established....");
        self.log_info("If the password is incorrect you may get banned from the server for too many failed authentication attempts");
        self.log_info("If you are banned you will have to un-ban the IP address from the server by using a client or restart it.\n");
    }

    pub fn log_welcome(&mut self){
        print!("\x1B[2J\x1B[1;1H"); // clears terminal
        println!("{} {} {}\n",Colour::Red.paint("[~] House of Iot"),Colour::Green.paint("CLI interface"), Colour::Red.paint("Version 1.0.0"));
        println!("\n Have any issues with this tool? Please report them to https://github.com/House-of-IoT/HOI-CLI/issues");
    }
    
    pub fn log_interval_data(&mut self, data:Facing,servers_name:String){
        println!("{}'s real time data",servers_name);
        self.print_sep_line("\n Config");
        self.print_row("Activating", data.config.activating.to_string());
        self.print_row("Deactivating", data.config.deactivating.to_string());
        self.print_row("Disconnecting" , data.config.disconnecting.to_string());
        self.print_row("Viewing", data.config.viewing.to_string());
        self.print_sep_line("\n Devices");
        self.print_row("Different types of bots" , data.different_bots.to_string());
        self.print_row("Non-Bots", data.non_bots.to_string());
        self.print_sep_line("\n Contacts");
        self.print_row("Number Of Contacts",data.contacts.len().to_string());
        self.print_sep_line("\n Networking");
        self.print_row("Banned ips" , data.banned_ips.len().to_string());
        self.print_row("Connection String" , data.connection_string);
    }

    pub fn print_row(&mut self,header:&str, data: String){
        println!("     {} - {}",Colour::Green.paint(header),data);
    }

    pub fn print_sep_line(&mut self, header:&str){
        println!("{}" , Colour::Blue.paint(header));
    }

}