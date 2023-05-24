from typing import Optional

from enum import Enum

class Currency(Enum):
    Platinum = 0,
    Gold = 1,
    Silver = 2,
    Copper = 3
    
    def __str__(self) -> str:
        return f"{self.name}"

currency_value = {Currency.Platinum: 1000, Currency.Gold : 100, Currency.Silver : 10, Currency.Copper : 1}
FILE = "files/partyfund.txt"

VALID_CURRENCY = {
    Currency.Platinum: ["Platinum", "p", "P", "PP", "pp"],
    Currency.Gold : ["Gold", "g", "G", "GP", "gp"],
    Currency.Silver : ["Silver", "s", "S", "SP", "sp"],
    Currency.Copper : ["Copper", "c", "C", "CP", "cp"]
}

# TODO: Add item values to total value

class Funds:
    funds = {Currency.Platinum : 0, Currency.Gold : 0, Currency.Silver : 0, Currency.Copper : 0}
    key_list = list(funds.keys())
    
    def __init__(self):
        self.load_funds()
    
    def __str__(self):
        pp = self.funds[Currency.Platinum]
        gp = self.funds[Currency.Gold]
        sp = self.funds[Currency.Silver]
        cp = self.funds[Currency.Copper]
        return "Platinum : {0} \nGold : {1} \nSilver : {2} \nCopper : {3}".format(pp, gp, sp, cp)
    
    def __repr__(self):
        return("{0};{1};{2};{3}".format(self.funds[Currency.Platinum], self.funds[Currency.Gold],self.funds[Currency.Silver],self.funds[Currency.Copper]))
    
    # Loads the partyfunds from the text file
    def load_funds(self):
        """
        Loads the partyfunds from the text file files/partyfunds.txt
        """
        
        input = open(FILE).read().split(";")
        key_list = list(self.funds.keys())
        
        for i in range(len(input)):
            self.funds[key_list[i]] = int(input[i])

    def money_in(self, money: int, currency_from: Currency, currency_to: Currency) -> float:
        return money * (currency_value[currency_from] / currency_value[currency_to])
    
    def save_funds(self):
        """
        Updates the partyfunds.txt file with the new partyfunds values.
        """
        with open(FILE, "w", encoding="utf-8") as f:
            f.write(self.__repr__())
    
    def funds_in(self, currency: Currency) -> int:
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
    
    def add_funds(self, amount: int, currency: Currency) -> bool:
        if(amount >= 0):
            self.funds[currency] += amount
            self.save_funds()
            return True
        else:
            return False
        
    def take_funds(self, amount: int, currency: Currency) -> bool:
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

    def parse_money_input(self, inp: str) -> tuple[int, Currency]:
        """Parses user input, into a valid money amount, and currency type

        Args:
            inp (str): User input

        Raises:
            Exception: If input is invalid, rasies an exception

        Returns:
            tuple[int, Currency]: money, currency type
        """
        m_str = ""
        currency = ""
        for c in inp:
            if c.isdigit():
                m_str += c
            else:
                currency += c
                
        
        currency = get_valid_currency(currency)
        
        if currency is None:
            raise Exception(f"Invalid argument: {inp}")
        
        return int(m_str), currency
        



def get_valid_currency(s: str) -> Optional[Currency]:
    """Checks if the given string, is a valid currency string

    Args:
        s (str): String to check

    Returns:
        (optional, str): The valid currency
    """
    for k, v in VALID_CURRENCY.items():
        if s in v:
            return k
    return None


def is_valid_currency(s: str) -> bool:
    """Checks if the given string, is a valid currency string

    Args:
        s (str): String to check

    Returns:
        bool: If its a valid currency symbol or not
    """
    for _, v in VALID_CURRENCY.items():
        if s in v:
            return True
    return False
            