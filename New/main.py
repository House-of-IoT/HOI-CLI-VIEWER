from facing_data import Config, Facing
from console_logging import ConsoleLogger
from request_handler import RequestHandler
import websockets
import asyncio
import json
import datetime

class Main:
    def __init__(self):
        self.logger = ConsoleLogger()
        self.password = None
        self.admin_password = None
        self.super_admin_password = None
        self.host = None
        self.port = None
        self.request_handler = None
        self.name = None
        self.servers_name = ""
        self.interval = None

    async def main(self ,restart = False):
        self.logger.start_message("HOI Analyzation Tool")
        if restart != True:
            self.password = input("\nPassword for the server:")
            self.admin_password = input("\nAdmin Password for the server:")
            self.super_admin_password = input("\nSuper Admin Password for the server:")
            self.host = input("\nHost:")
            self.port = input("\nPort:")
            self.name = input("\nName:")
            self.servers_name += input("\nServer name:")
            self.interval = int(input("\nData Gather Interval:"))

        websocket = await self.establish_connection()
        self.request_handler = RequestHandler(websocket,self)
        await websocket.send(self.password)
        await websocket.send(self.name_and_type())
        await websocket.send(self.servers_name)
        connection_response = await websocket.recv()

        if connection_response != "success":
            self.logger.log_failed_auth()
        else:
            self.logger.log_passed_auth()
            t1 = loop.create_task(self.begin_logging_information())
            await asyncio.wait([t1])

    
    async def establish_connection(self):
            times_attempted = 1
            try:
                return await websockets.connect(f'ws://{self.host}:{self.port}'  ,  ping_interval= None  , max_size = 20000000)
            except Exception as e:
                print(e)
                ConsoleLogger.log_issue_establishing_connection(times_attempted)
                times_attempted += 1

    async def gather_all_data_for_interval(self):
        config = await self.request_data_and_parse_config("server_config")
        contacts =await self.request_data_and_parse("contact_list", "array")
        banned_ips = await self.request_data_and_parse("servers_banned_ips","array")
        all_devices = await self.request_data_and_parse("servers_devices","map")
        deactivated_bots = await self.request_data_and_parse("servers_deactivated_bots","array")
        last_updated = datetime.datetime.now()
        facing = Facing(all_devices,config,json.loads(contacts),banned_ips,deactivated_bots)
        facing.analyze_data_and_populate_instance()
        ConsoleLogger.log_real_time_iteration(facing,self.servers_name,self.interval,last_updated)

    async def begin_logging_information(self):
        while True:
            await self.gather_all_data_for_interval()
            await asyncio.sleep(self.interval)

    async def request_data_and_parse_config(self,opcode):
        data = await  self.request_handler.handle_two_way_request(opcode)
        data_dict = json.loads(data)
        if data_dict["status"] == "success":
            config_dict = json.loads(data_dict["target_value"])
            config = Config(
                deactivating = config_dict["deactivating"], 
                activating=config_dict["activating"],
                disconnecting=config_dict["disconnecting"],
                viewing=config_dict["viewing"])
            return config
        else:
            return None

    async def request_data_and_parse(self,opcode,type_of_data):
        data = await self.request_handler.handle_two_way_request(opcode)
        data_dict = json.loads(data)
        if data_dict["status"] == "success":
            if type_of_data == "map":
                target_data = json.loads(data_dict["target_value"])
            else:
                target_data = data_dict["target_value"]
            return target_data
        else:
            return None

    def name_and_type(self):
        data = {"name":self.name , "type":"non-bot"}
        return json.dumps(data)           
 
if __name__ == "__main__":
    main = Main()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main.main())