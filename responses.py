from bag_of_holding import BagOfHolding
from funds import Funds

call_names = set(["reginald","regi", "rabbit", "hare"])
player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}
scorch_greetings = ["Please be careful today scorch, i do not like my hair scorched...", "Scorch! stay away with that blasted fire!", "Good Morning Scorch!"]


BAG_COMMANDS = [
    ("COMMAND <argument>", "Command setup"),
    ("add <item name or item id> <optional amount, default 1> <optional cost in gp, default 0> <optional weight in lbs, default 0>", "Adds the specified item to the bag"),
    ("remove <item name or item id> <optional amount, default 1>", "Removes an instance of the item from the bag of holding"),
    ("check <item name or item id or all>", "Shows the status of the specified item")
]

FUNDS_COMMAND = [
    ("COMMAND <argument>", "Command setup"),
    ("in <money>", "Shows how much money we got, in the specified currency"),
    ("add <money>", "Adds the given amount of money"),
    ("take <money>", "Takes out the given amount of money")
]

def handle_response(username, message, bag: BagOfHolding, funds: Funds) -> tuple[str, BagOfHolding, Funds]:
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    p_message = message.lower().split(" ")
    
    cmd, c_type, *args = p_message
    
    if cmd in call_names:
        match c_type:
            # Money
            case "funds":
                # If the length is two, return the funds
                if(len(p_message) == 2):
                    ... # Return the funds
                match args:
                    # Returns the funds in chosen currency
                    case ["in", money_arg]:
                        if funds.is_valid_currency(money_arg):
                            currency = funds.get_valid_currency(money_arg)
                            return f"We have {funds.funds_in(currency)} {currency}", bag, funds
                        return f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds
                    case ["add", money_arg]:
                        try:
                            money, m_type = funds.parse_money_input(money_arg)
                            funds.add_funds(money, m_type)
                            return f"Added {money} {m_type}", bag, funds
                        except Exception:
                            return f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds
                    case ["take", money_arg]:
                        try:
                            money, m_type = funds.parse_money_input(money_arg)
                            funds.take_funds(money, m_type)
                            return f"Added {money} {m_type}", bag, funds
                        except Exception:
                            return f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds
                    case _:
                        cmds = "\n".join([f"Command: `{c}`\nInfo: `{i}`" for c, i in FUNDS_COMMAND])
                        return f"{cmds}", bag, funds
                        
            # Bag of holding
            case "bag":
                match args:
                    # Adds an item to the bag
                    case ["add", item_arg] | ["add", item_arg, *_]: 
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        if len(args) >= 3:
                            amount = args[2:][0] if len([args[2:]]) >= 1 else 1
                            cost = args[2:][1] if len(args[2:]) >= 2 else 0
                            weight = args[2:][2] if len(args[2:]) >= 3 else 0
                        else:
                            amount = 1
                            cost = 0
                            weight = 0
                        
                        if item is None and not "id:" in item_arg[0:3]:
                            
                            item_name = item_arg
                            
                            item = bag.add_new_item(item_name, # Name
                                             int(amount) if amount is not None else 1, # Amount
                                             int(cost) if cost is not None else 0, # Cost
                                             int(weight) if weight is not None else 0) # Weight

                            return f"Created new item: {item.item_name}", bag, funds
                        elif item is not None:
                            amount = int(amount) if amount is not None else 1
                            bag.add_n_items(item, amount)
                            
                            return f"Added {amount} new instances, of item {item.item_name}", bag, funds
                        
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
                        if len(items) == 0:
                            return "No items to check", bag, funds
                        return f"{items}", bag, funds
                    
                    # Check the specified item
                    case ["check", item_arg]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return f"Could not find an instance of the specified item: {item_arg}", bag, funds
                        
                        return str(item), bag, funds
                    case _:
                        cmds = "\n".join([f"Command: `{c}`\nInfo: `{i}`" for c, i in BAG_COMMANDS])
                        return f"{cmds}", bag, funds
                        
    return f"Command did not match any known command: '{message}' '{args}'", bag, funds