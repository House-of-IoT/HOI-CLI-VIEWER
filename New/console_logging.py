from termcolor import colored

class ConsoleLogger:
    def __init__(self):
        self.row_number = 0

    def start_message(self , middle):
        print("\x1B[2J\x1B[1;1H") #console cleared
        print(colored("[~] House of Iot ", "red") + colored(middle,"green") +colored("Version 1.0.0\n","red"))
        print(colored("Source code: https://github.com/House-of-IoT\n"))
        print(colored("Got an issue?: https://github.com/House-of-IoT/HOI-GeneralServer/issues\n" , "green"))

    def log_info(self, data):
        print(colored("[Info] ~ ","yellow") + data)

    def log_generic_row(self,data,color):
        print(colored(f"\n[{self.row_number}] ~ ",color) + data)
        self.row_number += 1

    def log_config_success(self):
        self.log_generic_row("Successfully Created Config!\n" , "green")
        self.log_info("Usually the next step in setting up the analyzation tool is to run `python3 main.py`")
        input("press any key to exit....")

    def log_failed_auth(self):
        self.log_generic_row("Authentication Failed!", "red")
        self.log_info("If you just disconnected within the last 10 seconds , wait 10 seconds.")
        self.log_info("You may be blocked from the server if you attempted too many requests! An admin would need to remove your ban")

    def log_passed_auth(self):
        self.log_generic_row("Successfully Connected to server!", "green")

    @staticmethod
    def log_real_time_iteration(facing_data,server_name,interval,last_updated):
        print("\x1B[2J\x1B[1;1H")
        print(colored("[~] House of Iot ", "red") + colored("Data Analyzation Tool ","green") +colored("Version 1.0.0\n","red"))
        
        ConsoleLogger.log_servers_name(server_name,interval,last_updated)
        ConsoleLogger.log_header("Config")
        ConsoleLogger.log_sub_data("Deactivating",facing_data.config.deactivating)
        ConsoleLogger.log_sub_data("Activating",facing_data.config.activating)
        ConsoleLogger.log_sub_data("Disconnecting",facing_data.config.disconnecting)
        ConsoleLogger.log_sub_data("Viewing", facing_data.config.viewing)
        ConsoleLogger.log_header("Devices")
        ConsoleLogger.log_sub_data("Number of Bots",facing_data.num_of_bots)
        ConsoleLogger.log_sub_data("Number of Non-bots",facing_data.num_of_non_bots)
        ConsoleLogger.log_sub_data("Deactivated Bots", facing_data.num_of_deactivated_bots)
        ConsoleLogger.log_header("Networking")
        ConsoleLogger.log_sub_data("Number of Banned Ips", facing_data.num_of_banned_ips)
        ConsoleLogger.log_all(facing_data.banned_ips,"All Banned Ips")
        ConsoleLogger.log_header("Contacts")
        ConsoleLogger.log_sub_data("Number Of Contacts",facing_data.num_of_contacts)
        ConsoleLogger.log_all(facing_data.contacts,"All Contacts")
        ConsoleLogger.log_header("Relations")
        ConsoleLogger.log_all(facing_data.relation_num_list, "Num/Device")
        ConsoleLogger.log_sub_data("Number Of Relations",facing_data.num_of_relations)
        
    @staticmethod
    def log_no_config():
        print(colored("Couldn't locate a config file!","red"))
        input("press any key to exit...")
        quit()
    @staticmethod 
    def log_header(header):
        print("  "+colored(header,"blue"))

    @staticmethod
    def log_sub_data(prefix,data):
        data = str(data)
        print("      "+ colored(f"{prefix} - ","green") +data)
        
    @staticmethod 
    def log_servers_name(name,interval,last_updated):
        print(colored(f"{name}'s Real-Time Data!\n","white") + colored(f"\n  Current Data Interval: {interval}s\n","yellow"))
        print(colored(f"  Last Updated:{last_updated}\n","yellow"))

    @staticmethod
    def log_connection_issue():
        print(colored("Got a websocket error!!","red"))

    @staticmethod 
    def log_all(list_of_all,sub_header):
        print("      "+ colored(f"{sub_header}:","green"))
        for item in list_of_all:
            item = str(item)
            print("           " +item)
            
    @staticmethod
    def log_issue_establishing_connection(num):
        print(colored(f"[Times Attempted:{num}]issue establishing connection","red"))