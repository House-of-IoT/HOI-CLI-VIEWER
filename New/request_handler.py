import json 

class RequestHandler:
    def __init__(self,websocket,parent):
        self.parent = parent
        self.websocket = websocket

    async def handle_two_way_request(self,message):
        await self.websocket.send(message)
        response = await self.websocket.recv()
        basic_response = json.loads(response)

        #send authentication credentials if needed
        if basic_response["status"] == "needs-admin-auth":
            password_to_send = None
            if basic_response["action"] == "viewing":
                password_to_send = self.parent.admin_password
            else:
                password_to_send = self.parent.super_admin_password

            #return second response as string after auth attempt
            await self.websocket.send(password_to_send)
            second_response = await self.websocket.recv()
            return second_response
        else:
            return response
