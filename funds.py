currency_value = {"Platinum": 1000, "Gold" : 100, "Silver" : 10, "Copper" : 1}

class funds:
    funds = {"Platinum" : 0, "Gold" : 0, "Silver" : 0, "Copper" : 0}
    key_list = list(funds.keys())
    
    def __init__(self):
        self.load_funds()
    
    def __str__(self):
        pass
    
    def __repr__(self):
        return(self.funds["Platinum"],self.funds["Gold"], self.funds["Silver"],self.funds["Copper"])
    
    # Loads the partyfunds from the text file
    def load_funds(self):
        """
        Loads the partyfunds from the text file files/partyfunds.txt
        """
        
        input = open("files/partyfund.txt").read().split(" ")
        key_list = list(self.funds.keys())
        
        for i in range(len(input)):
            self.funds[key_list[i]] = int(input[i])
    
    # Updates the partyfunds.txt file with new values
    def store_funds(self, amount, currency):
        pass
    
    def funds_in(self, currency, item_values = False):
        """
            Show the partyfunds in the selected currency
        Args:
            currency (string): currency type
            item_values (bool): whether items value should be included in the calculation
        """
        amount = 0
        for key in self.funds.keys():
            amount += self.funds[key] * (currency_value[key]/currency_value[currency])
        return int(amount)
    
    def add_funds(self, amount, currency):
        if(amount >= 0):
            self.funds[currency] += amount
            return True
        else:
            return False
    
    def convert_funds():
        pass
    
    def take_funds(self, amount, currency):
        needed_amount = amount
        # Cant afford to take the amount
        if(self.funds_in(currency) < amount):
            return False
        
        if(self.funds[currency] < amount):
            # How much currency is used
            
            needed_amount -= self.funds[currency]
            self.funds[currency] = 0
            
            i = 0
            keys = self.key_list
            keys.reverse()
            while needed_amount > 0:  
                needed_amount -= self.funds[currency]
                self.funds[currency] = 0
                higher_currency = keys[self.key_list.index(currency)+i]
                if(self.funds[higher_currency] != 0):
                    self.funds[higher_currency] -= 1
                    lower_currency = self.key_list[self.key_list.index(currency)+i-1]
                    self.funds[lower_currency] += currency_value[higher_currency]/currency_value[lower_currency]
                    i-=1
                else:
                    i+=1
            self.funds[currency] += abs(needed_amount)
            
            print(self.funds,"|||")
            
                    
        if(amount >= 0):
            self.funds[currency] -= amount
            return True
        return False
    
vault = funds()

print(vault.__repr__())
print(vault.funds_in("Platinum"))

print(vault.funds)
print(vault.take_funds(10, "Copper"))