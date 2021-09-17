from termcolor import colored

class ConsoleLogger:

    def __init__(self):
        self.row_number = 0

    def start_message(self , middle):
        print("\x1B[2J\x1B[1;1H") #console cleared
        print("Server Started....\n")
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
    def log_no_config():
        print(colored("Couldn't locate a config file!","red"))
        input("press any key to exit...")
        quit()

    @staticmethod
    def log_connection_issue():
        print(colored("Got a websocket error!!","red"))

    @staticmethod
    def log_issue_establishing_connection(num):
        print(colored(f"[Times Attempted:{num}]issue establishing connection","red"))