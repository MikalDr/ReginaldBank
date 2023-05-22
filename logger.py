from datetime import datetime 
FILE = "files/log.txt"

class Log:
    entries = []
    def __init__(self):
        self.load_log()
    
    def __str__(self):
        entry_list = ""
        for entry in self.entries:
            entry_list += f"{entry[0]} | {entry[1]} {entry[2]} {entry[3]}" + "\n"
        return entry_list
    
    def __repr__(self):
        entry_list = ""
        for entry in self.entries:
            if entry == "":
                continue
            entry_list += ";".join(entry) + "\n"
        return entry_list
    
    def load_log(self):
        file = open(FILE).read().split("\n")
        for entry in file:
            if(entry == ""):
                continue
            date, user, action, amount, year = entry.split(";")
            self.entries.append([date,user,action,amount, year])
            
    def save_log(self):
        """
        Updates the partyfunds.txt file with the new partyfunds values.
        """
        with open(FILE, "w", encoding="utf-8") as f:
            f.write(self.__repr__())
    
    
    def add_log(self, user, action, amount):
        log = [datetime.now().strftime("%d.%m %H:%M"), user, action, amount, str(datetime.now().year)]
        self.entries.append(log)
        self.save_log()