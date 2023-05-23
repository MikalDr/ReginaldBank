from datetime import datetime 
FILE = "files/log.txt"

damage_emoji = {"Scorch" : "🔥", "Kerke" : "🪓", "Zenith" : "💥", "Reginald": "👊", "Marxson" : "🔨", "Halfdan" : "🎶"}

class Log:
    entries = []
    damage_log = []
    def __init__(self):
        self.load_log()
    
    def __str__(self):
        entry_list = ""
        for entry in self.entries:
            entry_list += f"{entry[0]} | {entry[1]} {entry[2]} {entry[3]}" + "\n"
        return entry_list
    
    def __repr__(self):
        entry_list = ""
        damage_list = ""
        # Spending log
        for entry in self.entries:
            if entry == "":
                continue
            entry_list += ";".join(entry) + "\n"
        # Damage log
        for entry in self.damage_log:
            if entry == "":
                continue
            damage_list += ";".join(entry) + "\n"
        
        return f"{entry_list}"#"{damage_list}"
    
    def load_log(self):
        file = open(FILE).read()
        if "#" not in file:
            return
        file = file.split("#")
        
        for entry in file[0].split("\n"):
            if(entry == ""):
                continue
            date, user, action, amount, year = entry.split(";")
            self.entries.append([date,user,action,amount, year])
        for entry in file[1].split("\n"):
            if(entry == ""):
                continue
            date, user, amount, target = entry.split(";")
            self.damage_log.append([date,user,amount,target])
            
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
    
    def show_log(self):
        response = ""
        for entry in self.entries:
            action = entry[2]
            if entry[2] == "TAKE":
                action = "took"
            if entry[2] == "ADD":
                action = "added"
            response += f"{entry[0]} | {entry[1]} {action} {entry[3]}\n"
            
        return response
    
    def show_damage_log(self):
        response = ""
        for entry in self.damage_log:
            target = "-> " + entry[3] + " "
            if(entry[3] == "None"):
                target = ""
            emoji = damage_emoji[entry[1]]
            response += f"{entry[0]} | {entry[1]} {target}({entry[2]}{emoji})\n"
        return response
    
    def add_damage(self, user, amount, target = "None"):
        log = [datetime.now().strftime("%d.%m %H:%M"), user, str(amount), target]
        self.damage_log.append(log)
        self.save_log()
        
l = Log()