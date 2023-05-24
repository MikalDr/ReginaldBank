from bag_of_holding import BagOfHolding, find_probable_approximations
from funds import Funds, is_valid_currency, get_valid_currency
from logger import Log
import random

call_names = set(["reginald","regi", "rabbit", "hare"])
player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}

SCORCH_GREETINGS = ["Please be careful today Scorch, i do not like my hair scorched...", "Scorch! stay away with that blasted fire!", "Note that i like my hair, not burnt", "I'll forgive you this once since you're but a child"]

ZENITH_GREETINGS = ["I always had a fantasy that elves were perfect being... well used to,", "Written anything nice recently?", "What does you book say about my kind?"]

REGINALD_GREETINGS = ["Fancy seeing you", "Always a pleasure", "You look ravishing today", ""]

KERKE_GREETINGS = ["Happy to see you!", "Have you been working out recently?", "Nice bod"]

HALFDAN_GREETINGS = ["I've been meaning to speak to you about the youth potion,", "Want to have a duel? your bajo my shawm?"]

MARXSON_GREETINGS = ["Glad we see eye to eye, you and i", "Beard looking as majestic as always", "May Moradin be with you", "Happy to see you!"]

MORAGO_GREETINGS = ["Please do not smite me today, oh", ""]

DEFAULT_GREETINGS = ["Greetings!", "Hey!", "Hi!", "Hello!"]

GREETING = {
    "Zenith": ZENITH_GREETINGS,
    "Scorch": SCORCH_GREETINGS,
    "me": REGINALD_GREETINGS,
    "Kerke": KERKE_GREETINGS,
    "Halfdan": HALFDAN_GREETINGS,
    "Marxson": MARXSON_GREETINGS,
    "Strange Godlike Being": MORAGO_GREETINGS
}


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
    ("take <money>", "Takes out the given amount of money"),
    ("log", "Shows the TODO")
]

LOGGER_COMMANDS = [
    ("COMMAND <argument>", "Command setup"),
    ("add <user> <amount> <target, optional>", "Adds the damage the specified user did to the possible target"),
    ("check", "Shows the damage log")
]

def handle_response(username, message, bag: BagOfHolding, funds: Funds, log: Log) -> tuple[str, BagOfHolding, Funds, Log]:
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    p_message = message.lower().split(" ")
    greeting = ""
    # Personal message time
    if name is not None:
        greeting = random.choice([f"{g} {name}" for g in DEFAULT_GREETINGS] + GREETING[name] if name in GREETING.keys() else [])
        greeting += "\n" 
        if(p_message == 1):
            return greeting, bag, funds, log        
    
    cmd, c_type, *args = p_message
    
    if cmd in call_names:
        match c_type:
            # Money
            case "funds":
                # If the length is two, return greeting +   the funds
                if(len(p_message) == 2):
                    ... # Return the funds
                match args:
                    case ["check"]:
                        return f"We have: ```\n{funds.__str__()}```", bag, funds, log

                    # Returns the funds in chosen currency
                    case ["in", money_arg]:
                        currency = get_valid_currency(money_arg)
                        if currency is not None:
                            return greeting + f"We have {funds.funds_in(currency)} {currency}", bag, funds, log
                        return greeting +   f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds, log
                    case ["add", money_arg]:
                        try:
                            money, m_type = funds.parse_money_input(money_arg)
                            
                            funds.add_funds(money, m_type)
                            log.add_log(name, "ADD", str(money)+" "+str(m_type))
                        except Exception:
                            return greeting +   f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds, log
                        return greeting +   f"Added {money} {m_type}", bag, funds, log
                    case ["take", money_arg]:
                        try:
                            money, m_type = funds.parse_money_input(money_arg)
                            funds.take_funds(money, m_type)
                            log.add_log(name, "TAKE", str(money)+" "+str(m_type))
                            return greeting +   f"took {money} {m_type}", bag, funds, log
                        except Exception:
                            return greeting +   f"I'm sorry, I did not understand what you meant, by {money_arg}", bag, funds, log
                    
                    case ["log"]:
                        return greeting +   log.show_log(), bag, funds, log
                    
                    case ["help"]:
                        cmds = "".join([f"```{c} \n- {i} ```" for c, i in FUNDS_COMMAND])
                        return greeting +   f"{cmds}", bag, funds, log
                        
            # Bag of holding
            case "bag":
                return f"", bag, funds, log
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

                            if cost is not None and funds.is_valid_currency(cost): # type: ignore
                                money, currency = funds.parse_money_input(cost) # type: ignore
                                cost = funds.money_in(money, currency, "Gold")
                            
                            item = bag.add_new_item(item_name, # Name
                                             int(amount) if amount is not None else 1, # Amount
                                             int(cost) if cost is not None else 0, # Cost
                                             int(weight) if weight is not None else 0) # Weight

                            return greeting +   f"Created new item: {item.item_name}", bag, funds, log
                        elif item is not None:
                            amount = int(amount) if amount is not None else 1
                            bag.add_n_items(item, amount)
                            
                            return greeting +   f"Added {amount} new instances, of item {item.item_name}", bag, funds, log
                        
                        return greeting +   f"Invalid args: {args}", bag, funds, log

                    # Removes an instance from the bag
                    case ["remove", item_arg, amount]:
                        
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return greeting +   f"Invalid args: {args}", bag, funds, log
                        try:
                            if bag.remove_item(item, int(amount) if amount is not None else 1):
                                return greeting +   f"Removed an instance of {item.item_name}", bag, funds, log
                            else:
                                return greeting +   f"Could not find an instance of the specified item", bag, funds, log
                        except Exception:
                            return greeting +   f"Could not remove the specified item, {item.item_name}, as there are not enough instances", bag, funds, log
                        
                    # Gets the total of a stat
                    case ["total", type, arg]:
                        if type in ["cost", "costs", "value", "worth"]:
                            return greeting +   f"The total cost of the items in the bag: {bag.get_total_worth(int(arg) if arg is not None else 0)} gp.", bag, funds, log
                        if type in ["weight", "weights", "lbs"]:
                            return greeting +   f"The total weight of the items in the bag: {bag.get_total_weight(int(arg) if arg is not None else 0)} lbs.", bag, funds, log
                        
                    # Deletes an item from the bag
                    case ["delete", item_arg]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return greeting +   f"Could not find an instance of the specified item: {item_arg}", bag, funds, log
                        
                        if bag.delete_item(item):
                            return greeting +   f"Deleted {item.item_name} from the bag.", bag, funds, log
                        else:
                            return greeting +   f"Could not find an instance of the specified item: {item_arg}", bag, funds, log
                    
                    # Checks the stats of all items
                    case ["check", "all"]:
                        items = '\n'.join([str(item) for item in bag.storage if item.amount >= 1])
                        if len(items) == 0:
                            return greeting +   "No items to check", bag, funds, log
                        return greeting +   f"{items}", bag, funds, log
                    
                    # Check the specified item
                    case ["check", item_arg]:
                        item = bag.get_item(item_id=item_arg.split(":")[1] if "id:" in item_arg[0:3] else None, item_name=item_arg)
                        
                        if item is None:
                            return greeting +   f"Could not find an instance of the specified item: {item_arg}", bag, funds, log
                        
                        return greeting +   str(item), bag, funds, log
                    case ["help"]:
                        cmds = "".join([f"```{c} \n- {i} ```" for c, i in BAG_COMMANDS])
                        return greeting +   f"{cmds}", bag, funds, log
            case "log":
                match args:
                    case ["add", user, amount]:
                        target = "None"
                        if len(args) == 4:
                            target = args[3]
                            
                        if not user.lower() in [k.lower() for k in GREETING.keys()]:
                            return greeting +   f"Who is {user}?", bag, funds, log
                        
                        if not target not in "None" and target.lower() in [k.lower() for k in GREETING.keys()]:
                            return greeting +   f"Who is {target}?", bag, funds, log
                        
                        if not amount.isdigit():
                            return greeting +   f"How can one do {amount} damage?", bag, funds, log
                        
                        log.add_damage(user, amount, target)
                        
                        msg = "Added {user} did {amount}" + f" to {target}" if target not in "None" else ""
                        
                        return greeting +   msg, bag, funds, log
                    
                    case ["check"]:
                        return greeting +   log.show_damage_log(), bag, funds, log
                    
                    case ["help"]:
                        cmds = "".join([f"```{c} \n- {i} ```" for c, i in LOGGER_COMMANDS])
                        return greeting +   f"{cmds}", bag, funds, log

            case "help":
                message = "```1. regi funds help \n2. regi bag help\n3. regi log help``` "
                return greeting + f"Sure thing! how can i assist you?" + message, bag, funds, log
            
            case "commands":
                return greeting + "\n".join([f"Command: `{c}`\nInfo: `{i}`" for c, i in BAG_COMMANDS + FUNDS_COMMAND + LOGGER_COMMANDS]), bag, funds, log

    return greeting +   f"Command did not match any known command: ´{message}´", bag, funds, log