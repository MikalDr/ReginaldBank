FILE = "files/config.txt"

class Config:
    def __init__(self):
        pass

    def __repr__(self):
        return ";".join(self.channels)
    
    def save_config(self):
        with open(FILE, "w", encoding="utf-8") as f:
            f.write(self.__repr__())

    def get_channels(self):
        return self.channels

    def load_config(self):
        input = open(FILE).read().split(";")
        self.channels = input
    
    def add_channel(self, channel_name):
        if(not(channel_name in self.channels)):
            self.channels.append(channel_name)
        self.save_config()

con = Config()
con.load_config()

print(con.__repr__())
