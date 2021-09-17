from console_logging import ConsoleLogger
import websockets
import asyncio

class Main:
    def __init__(self):
        self.logger = ConsoleLogger()
        self.password = None
        self.admin_password = None
        self.super_admin_password = None
        
    async def main(self ,restart = False):
        self.logger.start_message("Door Monitor")
        if restart != True:
            self.password = input("\nPassword for the server: ")
        websocket = await self.establish_connection()
        await websocket.send(self.password)
        await websocket.send(self.name_and_type())
        await websocket.send("main")
        connection_response = await websocket.recv()

        if connection_response != "success":
            self.logger.log_failed_auth()
        else:
            self.logger.log_passed_auth()
            t1 = loop.create_task(self.test_send_periodic_data_and_listen(websocket))
            await asyncio.wait([t1])

    async def establish_connection(self):
        times_attempted = 1
        while True:
            try:
                return await websockets.connect(f'ws://{self.config.host}:{self.config.port}'  ,  ping_interval= None  , max_size = 20000000)
            except:
                ConsoleLogger.log_issue_establishing_connection(times_attempted)
                times_attempted += 1

    async def begin_logging_information(self):

                
                
 
if __name__ == "__main__":
    main = Main()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main.main())