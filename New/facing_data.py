class Facing:
    def __init__(self,all_devices = None,config = None , contacts = None , banned_ips = None, deactivated_bots = None):
        self.all_devices = all_devices
        self.config = config  
        self.contacts = contacts
        self.banned_ips = banned_ips
        self.deactivated_bots = deactivated_bots
        self.num_of_deactivated_bots = 0
        self.num_of_non_bots = 0
        self.num_of_bots = 0
        self.num_of_contacts = 0
        self.num_of_banned_ips = 0

    def analyze_data_and_populate_instance(self):
        self.num_of_banned_ips = len(self.banned_ips)
        self.num_of_contacts = len(self.contacts)
        self.deactivated_bots = len(self.num_of_deactivated_bots)
        self.gather_device_stats()

    def gather_device_stats(self):
        all_device_keys = self.all_devices.keys()
        for key in all_device_keys:
            if self.all_devices[key] == "non-bot":
                self.num_of_non_bots += 1
        self.num_of_bots = len(all_device_keys) - self.num_of_non_bots  
            
class Config:
    def __init__(self,deactivating = None,activating = None,disconnecting = None,viewing = None):
        self.deactivating = deactivating
        self.activating = activating
        self.disconnecting = disconnecting
        self.viewing = viewing