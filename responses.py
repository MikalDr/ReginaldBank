import math


class Funds:
    def __init__(self, plat, gold, electrum, silver, copper):
        self.plat = plat
        self.gold = gold
        self.electrum = electrum
        self.silver = silver
        self.copper = copper
    
    def __repr__(self):
        return f"{self.plat};{self.gold};{self.electrum};{self.silver};{self.copper}"

    def __str__(self):
        return (f"Plat: {self.plat} \n Gold: {self.gold} \n Electrum: {self.electrum} \n Silver: {self.silver} \n Copper: {self.copper}")
        return f"{self.plat};{self.gold};{self.electrum};{self.silver};{self.copper}"
    
    def update_fund(self, plat=0, gold=0, electrum=0, silver=0, copper=0):
        self.plat += plat
        self.gold += gold
        self.electrum += electrum
        self.silver += silver
        self.copper += copper

        f = open("partyfund.txt", "w")
        f.write(self.__repr__())
        
    def fund_in_currency(self, currency, partystorage=0):
        amount = self.silver * 10 + self.electrum * 50 + self.gold * 100 + self.plat*1000 + self.copper
        match currency:
            case "g":
                return math.floor(amount/100) + partystorage
            case "p":
                return math.floor(amount/1000) + partystorage
            case "s":
                return math.floor(amount/10) + partystorage
            case "e":
                return math.floor(amount/50) + partystorage
            case "c":
                return amount + partystorage

# Log of people using
class Logs:
    def __init__self(logs):
        pass

currency_values = {"p" : 1000, "g" : 100, "e" : 50, "s" : 10, "c" : 1}

class PartyStorage:
    def __init__(self, items):
        storage = {}
        id = 0
        for el in items:
            if el == "":
                continue
            quantity, item_name, cost = el.split(",")
            storage[id] = [item_name, quantity, cost, int(quantity)*int(cost[:-1])]
            id += 1
        self.storage = storage
    def __str__(self):
        library = ""
        for item in self.storage.keys():
            print(item)
            library += f"{self.storage[item][1]} | {self.storage[item][0]} ({self.storage[item][2]}) | Total value: {str(self.storage[item][3]) + self.storage[item][2][-1]}"
        return library
    
    def value_of_items(self, currency) -> int:
        amount = 0
        for item in self.storage.keys():
            print("test",self.storage[item][3], currency_values[self.storage[item][2][-1]])
            amount += (self.storage[item][3] * currency_values[self.storage[item][2][-1]])
        match(currency):
            case "g":
                return math.floor(amount/100)
            case "p":
                return math.floor(amount/1000)
            case "s":
                return math.floor(amount/10)
            case "e":
                return math.floor(amount/50)
            case "c":
                return amount

player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}

def handle_response(username, message) -> str:
    partyfund = load_partyfund()
    upper_message = message.split()
    p_message = message.lower()
    
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    
    partyfund = load_partyfund()
    storage = load_partystorage()
    
    if p_message.split()[0] == "reginald" or p_message.split()[0] == "regi" or p_message.split()[0] == "rabbit" or p_message.split()[0] == "hare":
        p_message = p_message.split()
        context = f"Sure thing {name}! \n"
        print(p_message)
        if(" ".join(p_message[1:]) == "check partyfund" or " ".join(p_message[1:]) == "check funds"):
            return context + "Oh, let me check the logs.... We currently have! \n {0}\n\n Items \n {1}".format(partyfund, storage)
        if(p_message[1] == "funds" and p_message[2] == "in"):
            return partyfund.fund_in_currency(p_message[3], storage.value_of_items(p_message[3]))
        if(p_message[1] == "withdraw"):
            ok = checkCurrency(partyfund, p_message[2:], negative=True)
            if(ok):
                if(name == "me"):
                    return context + "I have withdrawn {0} from the partyfund, we now have\n {1}".format(" ".join(upper_message[2:]), partyfund)
                return context + "You have withdrawn {0} from the partyfund, we now have\n {1} \nItems \n {2}".format(" ".join(upper_message[2:]), partyfund, storage)
            else:
                return context + "I am not sure if we have {0} in the partyfund!".format(" ".join(upper_message[2:]))
        if(p_message[1] == "add"):
            if(p_message[2] == "item"):
                pass
            else:
                ok = checkCurrency(partyfund, p_message[2:])
                if(ok):
                    partyfund = load_partyfund()
                    if(name == "me"):
                        return context +"Good job {2}! \nI have contributed {0} to the partyfund, we now have:\n {1}".format(" ".join(upper_message[2:]), partyfund, name)
                    return context +"Good job {2}! \nYou have contributed {0} to the partyfund, we now have:\n {1}".format(" ".join(upper_message[2:]), partyfund, name)
                else:
                    return context +"I may be a rabb.. Hare! But i am not adding {0} to the partyfund!".format(" ".join(upper_message[2:]))
        return context +"Sorry, could you repeat that?"
    
    if(p_message == "!help"):
        return (f"Hi {name}! Among my repetoire of talents, i can assist you with: \n" + 
                "+ reginald add x -> where x can be 100g or 100g 50c 4p \n"+
                "- reginald add item x -> where x is the quantity item and cost, like 20 Gems 50g\n" +
                "+ reginald withraw x -> where x is the amount of cash requested (for your money spending needs)\n" + 
                "- reginald withraw item x -> where x is the quantity and item, like 1 Gem\n"
                "+ reginald check funds -> i will check the funds i have hidden in Zeniths bag of holding (dont tell him)\n" + 
                "+ reginald funds in x -> where x is either p, g, e, s, c and i will tell you our total fund in that currency.\n" +
                "\n I may learn some new tricks later if you convince me to.") 
    
def checkCurrency(funds, strings, negative = False):
    amount = ""
    for string in strings:
        for char in string:
            if(char == "-"):
                return False
            if char.isdigit():
                amount += char
        if(amount == ""):
            return False
        amount = int(amount)
        if(amount < 0):
            return False
        if(negative):
            amount = -amount
        if("g" in string or "gp" in string):
            if(not negative or abs(amount) <= funds.gold):
                funds.update_fund(gold = amount)
                return True
            return False
        elif("s" in string or "sp" in string):
            if(not negative or abs(amount) <= funds.silver):
                funds.update_fund(silver = amount)
                return True
            return False
        elif("c" in string or "cp" in string):
            if(not negative or abs(amount) <= funds.copper):
                funds.update_fund(copper = amount)
                return True
            return False
        elif("e" in string or "ep" in string):
            if(not negative or abs(amount) <= funds.electrum):
                funds.update_fund(electrum = amount)
                return True
            return False
        elif("p" in string or "pp" in string):
            if(not negative or abs(amount) <= funds.plat):
                funds.update_fund(plat = amount)
                return True
            return False
        else:
            return False
     
def load_partyfund():
    file = open("partyfund.txt", "r").read()
    plat, gold, electrum, silver, copper = map(int, file.split(";"))
    funds = Funds(plat, gold, electrum, silver, copper)
    return funds

def score_board():
    rankings = []
    pass

def load_partystorage():
    file = open("partystorage.txt", "r").read()
    entries = file.split(";")
    return PartyStorage(entries)
    
storage = load_partystorage()

print(storage)