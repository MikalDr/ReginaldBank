from __future__ import annotations
from typing import Optional
import Levenshtein

FILE = "files/partystorage.csv"
HEADERS = "id;item_name;amount;cost;weight"
SEPERATOR = ";"

class Item:
    def __init__(self, item_name: str, amount: int, cost: int, item_weight: int = 0, item_id: Optional[int] = None) -> None:
        self.item_name = item_name
        """Name of the item"""
        self.amount = amount
        """How many instances of the item"""
        self.cost = cost
        """How much each item is worth, in gold"""
        self.item_weight = item_weight
        """Weight of the item, in lbs"""
        
        if item_id is None:
            item_id = get_id()
        self.item_id = item_id
        """Id of the item"""
        
    def get_csv_format(self) -> str:
        return f"{self.item_id}{SEPERATOR}{self.item_name}{SEPERATOR}{self.amount}{SEPERATOR}{self.cost}"
        
    def __str__(self) -> str:
        return f"{self.item_name} x{self.amount}"

class BagOfHolding:
    def __init__(self) -> None:
        self.storage: set[Item] = set()
        
        
    def load_items(self) -> None:
        """Loads all the items from the file, into the bag of holding, should be called on start"""
        with open(FILE, "r", encoding="utf-8") as f:
            for line in f.readlines()[1::]:
                item_id, item_name, amount, cost = line.split(SEPERATOR)
                self.storage.add(Item(item_name, int(amount), int(cost), int(item_id)))
    
    def add_n_items(self, item: Item, n: int) -> None:
        """Adds n instances of the specified item to the bag

        Args:
            item (Item): Item
            n (int): Instance count
        """
        for it in self.storage:
            if it.item_id == item.item_id:
                self.storage.remove(it)
                it.amount += n
                self.storage.add(it)
                return
                
    def save_items(self) -> None:
        """Saves all the items, should be called before exit"""
        with open(FILE, "w", encoding="utf-8") as f:
            f.write(HEADERS)
            f.writelines([item.get_csv_format() for item in self.storage])
       
            
    def get_total_worth(self, assumption: int = 0) -> int:
        """Returns the total cost of the items, in gp

        Args:
            assumption (int, optional): Some items have no specified cost, so use this value as a stand in, when calculating the total. Defaults to 0.
        
        Returns:
            int: total cost in gp
        """
        return sum([item.cost for item in self.storage]) + len([0 for item in self.storage if item.cost == 0]) * assumption
    
    
    def get_total_weight(self, assumption: int = 0) -> int:
        """Returns the total weight of the items in the bag.

        Args:
            assumption (int, optional): Some items have no specified weights, so use this value as a stand in, when calculating the total. Defaults to 0.

        Returns:
            int: Weight in lbs
        """
        return sum([item.item_weight for item in self.storage if item.item_weight != 0]) + len([0 for item in self.storage if item.item_weight == 0]) * assumption


    def get_item(self, item_id: Optional[int] = None, item_name: Optional[str] = None) -> Optional[Item]:
        """Gets an item, based on either the item id,
        or their item name.
        Args:
            item_id (Optional[int], optional): Item id. Defaults to None.
            item_name (Optional[str], optional): Item name. Defaults to None.

        Raises:
            Exception: If neither item_id or item_name is specified, raises an exception

        Returns:
            Optional[Item]: Possible item
        """
        if item_id == None and item_name == None:
            raise Exception(f"Expected either item_id or item_name, but got neither.")
        
        for item in self.storage:
            if item_id is not None and item.item_id == item_id:
                return item
            if item_name is not None and item_name in item.item_name:
                return item


    def get_item_names(self) -> list[str]:
        """Returns a list of all the names of the items in the bag

        Returns:
            list[str]: list of item names
        """
        return [item.item_name for item in self.storage]
    
    def get_item_by_cost(self, amount: int, min: Optional[int] = None, max: Optional[int] = None) -> list[Item]:
        """Gets all the items, that have the cost within the specifed range.

        Args:
            amount int: Amount in gold
            min (int, optional): Minimum gold cost. Defaults to amount.
            max (int, optional): Maximum gold cost. Defaults to amount.

        Returns:
            list[Item]: List of the items
        """
        if min is None:
            min = amount
        if max is None:
            max = amount
        return [item for item in self.storage if min <= item.cost <= max and item.amount >= 1]
    
    
    def remove_item(self, item: Item, amount: int = 1) -> bool:
        """Removes an instance of the specified item

        Args:
            item: Item
            amount (int): The amount of instances that will be removed

        Raises:
            Exception: If both item_id and item_name are None,
            or if the specified item does not have enough instances to remove
            
            
        Returns:
            bool: True if an instance was removed
        """
        pre_item = None
        for it in self.storage:
            if it.item_id == item.item_id:
                pre_item = it
                self.storage.remove(it)
                break
        
        if pre_item is None:
            return False
        
        if pre_item.amount < amount:
            raise Exception("Cannot remove more items than there are instances of the item.")
        
        pre_item.amount -= amount
        
        return True
    
    
    def add_new_item(self, item_name: str, amount: int = 1, cost: int = 0, item_weight: int = 0) -> Item:
        """Adds a new item, and returns the instances, to the storage

        Args:
            item_name (str): Item name
            amount (int, optional): Item instance count. Defaults to 1.
            cost (int, optional): Item cost, per instance. Defaults to 0.
            item_weight (int, optional): Weight of the item, in lbs, Defaults to 0.

        Returns:
            Item: Item
        """
        pre_item = self.get_item(item_name=item_name)
        
        if pre_item is None:
            item = Item(item_name, amount, cost, item_weight, get_id())
            self.storage.add(item)
            return item

        pre_item.amount += amount
        
        for item in self.storage:
            if item == pre_item:
                self.storage.remove(item)
                self.storage.add(pre_item)
                break
        return pre_item
    
    
    def add_item(self, item: Item) -> None:
        """Adds the specified item, if it already exists, adds the specified amount to the instance count.

        Args:
            item (Item): Item
        """
        pre_item = self.get_item(item_name=item.item_name)
        
        if pre_item is None:
            self.storage.add(item)
        else:
            pre_item.amount += item.amount
            for item in self.storage:
                if item == pre_item:
                    self.storage.remove(item)
                    self.storage.add(pre_item)
        
            
    def delete_item(self, item: Item) -> bool:
        """Removes the specified item from the storage

        Args:
            item (Item): Item

        Returns:
            bool: If an item was removed or not
        """
        l = len(self.storage)
        
        self.storage.remove(item)
        
        return l - len(self.storage) != 0
    
        
    def closest_approximate_item(self, item_name: str) -> list[str]:
        """Gets the closest approximate items

        Args:
            item_name str: Name of the possible item

        Returns:
            list[str]: List of probable approximations
        """
        return find_probable_approximations(item_name, self.get_item_names())




def find_probable_approximations(word: str, word_list: list[str], threshold: int = 2) -> list[str]:
    """Finds a probable appxorimation for the given word,
    by checking the it against the list of words,
    using the levenstein distance algoritm.

    Args:
        word str: Typed word
        word_list str: List of actual words
        threshold (int, optional): Threshold. Defaults to 2.

    Returns:
        list[str]: list of probable approximations
    """
    approximations = []
    for w in word_list:
        distance = Levenshtein.distance(word, w)
        if distance <= threshold:
            approximations.append(w)
    return approximations



def get_id() -> int:
    """Gets a new id, by reading the file,
    getting all their ids, and returning the max +1

    Returns:
        int: new id
    """
    with open(FILE, "r", encoding="utf-8") as f:
        return max([int(line.split(";")[0]) for line in f.readlines()[1::]]) + 1
            


if __name__ == "__main__":
    b = BagOfHolding()
    b.load_items()