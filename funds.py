currency_value = {"Platinum": 1000, "Gold" : 100, "Silver" : 10, "Copper" : 1}
FILE = "files/partyfund.txt"

# TODO: Add item values to total value

class Funds:
    funds = {"Platinum" : 0, "Gold" : 0, "Silver" : 0, "Copper" : 0}
    key_list = list(funds.keys())
    
    def __init__(self):
        self.load_funds()
    
    def __str__(self):
        pass
    
    def __repr__(self):
        return("{0};{1};{2};{3}".format(self.funds["Platinum"], self.funds["Gold"],self.funds["Silver"],self.funds["Copper"]))
    
    # Loads the partyfunds from the text file
    def load_funds(self):
        """
        Loads the partyfunds from the text file files/partyfunds.txt
        """
        
        input = open(FILE).read().split(";")
        key_list = list(self.funds.keys())
        
        for i in range(len(input)):
            self.funds[key_list[i]] = int(input[i])
    
    def save_funds(self):
        """
        Updates the partyfunds.txt file with the new partyfunds values.
        """
        with open(FILE, "w", encoding="utf-8") as f:
            f.write(self.__repr__())
    
    def funds_in(self, currency):
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
            self.save_funds()
            return True
        else:
            return False
        
    def take_funds(self, amount, currency):
        """
        Takes funds from the partyfunds, if there is not enough of selected currency
        it converts from higher currency if possible

        Args:
            amount (_type_): amount taken
            currency (_type_): selected currency

        Returns:
            bool: whether the process was handled or not 
        """
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
                    self.funds[lower_currency] += int(currency_value[higher_currency]/currency_value[lower_currency])
                    i-=1
                else:
                    i+=1
            self.funds[currency] += abs(needed_amount)
                    
        elif(amount >= 0):
            self.funds[currency] -= amount
            return True
        
        self.save_funds()
        return False