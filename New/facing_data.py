import json 
class Facing:
    def __init__(self,
    all_devices = None,
    config = None , 
    contacts = None , 
    banned_ips = None, 
    deactivated_bots = None, 
    passive_data = None):
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
        self.num_of_device_passive_data = 0
        self.passive_data = passive_data
        self.relation_num_list = []
        self.num_of_relations = 0

    def analyze_data_and_populate_instance(self):
        if self.banned_ips != None:
            self.num_of_banned_ips = len(self.banned_ips)
        if self.contacts != None:
            self.num_of_contacts = len(self.contacts.keys())
        if self.deactivated_bots != None:
            self.num_of_deactivated_bots = len(self.deactivated_bots)
        self.gather_device_stats()

    def gather_device_stats(self):
        if self.all_devices != None:
            all_device_keys = self.all_devices.keys()
            for key in all_device_keys:
                if self.all_devices[key] == "non-bot":
                    self.num_of_non_bots += 1
            self.num_of_bots = len(all_device_keys) - self.num_of_non_bots
        if self.passive_data != None:
            self.num_of_device_passive_data = self.passive_data["server_state_lens"]["in_memory_passive_data"]
            self.gather_relation_data()

    def gather_relation_data(self):
        relations = json.loads(self.passive_data["external_controller_snapshot"])["relations"]
        self.num_of_relations = len(relations)
        #find out how many relations each device have
        devices_and_relations_num = {}
        for relation in relations:
            device_name = relation["device_name"]
            if device_name in devices_and_relations_num:
                devices_and_relations_num[device_name] += 1
            else:
                devices_and_relations_num[device_name] = 1
        for device_name in devices_and_relations_num.keys():
            num_of_relations = devices_and_relations_num[device_name]
            self.relation_num_list.append(f"{device_name}->{num_of_relations}")
                
class Config:
    def __init__(self,deactivating = None,activating = None,disconnecting = None,viewing = None):
        self.deactivating = deactivating
        self.activating = activating
        self.disconnecting = disconnecting
        self.viewing = viewing