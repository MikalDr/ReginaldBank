from bag_of_holding import BagOfHolding
from funds import Funds

call_names = set(["reginald","regi", "rabbit", "hare"])
player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}
scorch_greetings = ["Please be careful today scorch, i do not like my hair scorched...", "Scorch! stay away with that blasted fire!", "Good Morning Scorch!"]


def handle_response(username, message, bag: BagOfHolding, funds: Funds) -> tuple[str, BagOfHolding, Funds]:
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    p_message = message.lower().split(" ")
    
    cmd, *args = p_message
    
    if cmd in call_names:
        match p_message[1]:
            # Money
            case "funds":
                # If the length is two, return the funds
                if(len(p_message) == 2):
                    ... # Return the funds
                match p_message[2]:
                    case "in":
                        ... # Returns the funds in chosen currency
                    case "add":
                        ... # Adds to the funds
                    case "take":
                        ... # Removes from the funds
                        
            # Bag of holding
            case "bag":
                match args:
                    # Adds an item to the bag
                    case ["add", item_arg, amount, cost, weight]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None and not "id:" in item_arg[0:3]:
                            
                            item_name = item_arg
                            
                            item = bag.add_new_item(item_name, # Name
                                             int(amount) if amount is not None else 1, # Amount
                                             int(cost) if cost is not None else 0, # Cost
                                             int(weight) if weight is not None else 0) # Weight

                            return f"Created new item: {item.item_name}", bag, funds
                        elif item is not None:
                            item.amount = int(amount) if amount is not None else 1
                            bag.add_item(item)
                            
                            return f"Added {item.amount} new instances, of item {item.item_name}", bag, funds
                        
                        return f"Invalid args: {args}", bag, funds

                    # Removes an instance from the bag
                    case ["remove", item_arg, amount]:
                        
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return f"Invalid args: {args}", bag, funds
                        try:
                            if bag.remove_item(item, int(amount) if amount is not None else 1):
                                return f"Removed an instance of {item.item_name}", bag, funds
                            else:
                                return f"Could not find an instance of the specified item", bag, funds
                        except Exception:
                            return f"Could not remove the specified item, {item.item_name}, as there are not enough instances", bag, funds
                        
                    # Gets the total of a stat
                    case ["total", type, arg]:
                        if type in ["cost", "costs", "value", "worth"]:
                            return f"The total cost of the items in the bag: {bag.get_total_worth(int(arg) if arg is not None else 0)} gp.", bag, funds
                        if type in ["weight", "weights", "lbs"]:
                            return f"The total weight of the items in the bag: {bag.get_total_weight(int(arg) if arg is not None else 0)} lbs.", bag, funds
                        
                    # Deletes an item from the bag
                    case ["delete", item_arg]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return f"Could not find an instance of the specified item: {item_arg}", bag, funds
                        
                        if bag.delete_item(item):
                            return f"Deleted {item.item_name} from the bag.", bag, funds
                        else:
                            return f"Could not find an instance of the specified item: {item_arg}", bag, funds
                    
                    # Checks the stats of all items
                    case ["check", "all"]:
                        items = '\n'.join([str(item) for item in bag.storage if item.amount >= 1])
                        return f"{items}", bag, funds
                    
                    # Check the specified item
                    case ["check", item_arg]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return f"Could not find an instance of the specified item: {item_arg}", bag, funds
                        
                        return str(item), bag, funds
                        
                        
                        
            

        return f"Unknown command: {message}", bag, funds