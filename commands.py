"""
commands.py

Parses and executes commands given by the players
"""


from bag_of_holding import BagOfHolding, find_probable_approximations
from funds import Funds, Currency, is_valid_currency
from logger import Log
import random

CALL_COMMANDS = set(["reginald","regi", "rabbit", "hare"])
PLAYER_TAGS = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}

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

COMMANDS = {
    "log" : {
        "args":"optional: session id/date",
        "example": "'regi log' or 'regi log 1'",
        "flags": "-dmg, -funds, -bag, -cmd",
        "info": "Displays the log"
    },
    "add" : {
        "args": "funds/item",
        "example": "'regi add 100gp' or 'regi add sword'",
        "flags": "--no-log",
        "info": "Adds gold or an item"
    },
    "take" : {
        "args": "funds/item",
        "example": "'regi take 100gp' or 'regi take sword'",
        "flags": "--no-log",
        "info": "Takes gold or an item"
    },
    "find" : {
        "args": "item name/item id",
        "example": "'regi find sword' or 'regi find 0'",
        "flags": "--user:username",
        "info": "Displays all items that match the catagory or id"
    },
    "funds" : {
        "args": "optional: session id/date",
        "example": "'regi funds' or 'regi funds 23.05.23'",
        "flags": "--user:username",
        "info": "Displays the funds"
    },
    "bag" : {
        "args": "optional: session id/date",
        "example": "'regi bag' or 'regi bag 23.05.23'",
        "flags": "--user:username",
        "info": "Displays the contents of the bag"
    },
    "help" : {
        "args": "optional: command/flag",
        "example": "'regi help add' or 'help \"-dmg\"''",
        "flags": "",
        "info": "Displays this"
    }
}

FLAGS = {
    "-dmg" : {
        "info": "Displays damage log",
        "verbose info": "Used as a flag for the 'log'-command, to ensure only the damage log is shown",
        "example": "'regi log -dmg'"
    },
    "-funds" : {
        "info": "Displays the funds log",
        "verbose info": "Used as a flag for the 'log'-command, to ensure only the funds log is shown",
        "example": "'regi log -funds'"
    },
    "-bag" : {
        "info": "Displays the bag log",
        "verbose info": "Used as a flag for the 'log'-command, to ensure only the bag log is shown",
        "example": "'regi log -bag'"
    },
    "-cmd" : {
        "info": "Displays the cmd log",
        "verbose info": "Used as a flag for the 'log'-command, to ensure only the cmd log is shown",
        "example": "'regi log -cmd'"
    },
    "--no-log" : {
        "info": "Does not log the command.",
        "verbose info": "Used as a flag for any command, mainly for debugging, to ensure that the command, and its effects will not be logged",
        "example": "'regi take 100gp --no-log'"
    },
    "--user:username" : {
        "info": "Displays parts of the log, that pertain to the specific user",
        "verbose info": "Used as a flag for the 'log'.command, to ensure only the parts of the log pertaining to the specific user is shown, is not case sensitive",
        "example": "'regi log -funds -user:Kerke'"
    },
    "--dry-run" : {
        "info": "Does not execute the given command",
        "verbose info": "Used as a flag for any command, and ensures that the given command is not executed, but acts like it. The command is nor its effect are logged",
        "example": "'regi take sword --dry-run'"
    },
    "-v" : {
        "info": "Displays the verbose information about a command or flag",
        "verbose info": "Displays the verbose information about a command or flag, but some commands have their normal info just repeated in 'verbose info'",
        "example": "'help take -v'"
    },
    "-example" : {
        "info": "Ensures a help command only displays the 'example'-part of the help result",
        "verbose info": "Ensures a help command only displays the 'example'-part of the help result",
        "example": "'help add -example'"
    }
}


def parse_command(username, message, bag: BagOfHolding, funds: Funds, log: Log) -> tuple[str, BagOfHolding, Funds, Log]:
    
    
    p_message = message.lower().split(" ")
    tag = username.split("#")[1]
    name = PLAYER_TAGS.get(tag)
    greeting = ""
    
    # Personal message
    if name is not None:
        greeting = random.choice([f"{g} {name}" for g in DEFAULT_GREETINGS] + GREETING[name] if name in GREETING.keys() else [])
        greeting += "\n" 
    
    if p_message[0] not in CALL_COMMANDS: # Message does not start with "regi"
        return "", bag, funds, log
    
    if len(p_message) == 1: # Message is not long enough, so just do a greeting instead.
        return greeting, bag, funds, log
    
    # We start at 1, since the first word is always 'regi'
    cmd = p_message[1]
    
    args, flags = get_flags_and_arguments(" ".join(p_message[2:])) # Get arguments and flag, ignoring the first two words
    
    match cmd:
        case "log":
            raise NotImplementedError
        
        # TODO: Add flags
        # Adds item or money
        case "add":
            
            # Player has only typed regi add
            if len(args) == 0:
                return pretty_command("add", flags), bag, funds, log
            
            m_money = args[0]
            
            # TODO: Fix edge case where user types '1g10p'
            if is_valid_currency("".join([c for c in m_money if not c.isdigit()])):
                # Add money
                money, currency = funds.parse_money_input(m_money)
                
                funds.add_funds(money, currency)
                
                # TODO: Implement logging, for funds
                # TODO: Implement logging, for commands
                                
                return f"Added {money}{currency} to our funds", bag, funds, log
            else:
                # Add item
                
                item_name = args[0]
                amount = 1
                cost = 0
                currency = Currency.Gold
                weight = 0
                
                try:
                    if len(args[1:]) >= 1:
                        amount = int(args[1])
                except ValueError:
                    return f"I am sorry, I haven't heard of the number '{args[1]}'", bag, funds, log

                if len(args[2:]) >= 1:
                    raw_cost = args[2]
                    
                    # TODO: Fix edge case where user types '1g10p'
                    if not is_valid_currency("".join([c for c in raw_cost if not c.isdigit()])):
                        return f"I am sorry, I haven't heard of the number '{args[1]}'", bag, funds, log
                    cost, currency = funds.parse_money_input(raw_cost)

                try:
                    if len(args[3:]) >= 1:
                        weight = int(args[3])
                except ValueError:
                    return f"I am sorry, I haven't heard of the number '{args[3]}'", bag, funds, log
                
                # TODO: Implement logging, for items
                # TODO: Implement logging, for commands
                
                item = bag.add_new_item(item_name, amount, cost, currency, weight)
                
                return f"Added {str(item)} to our bag", bag, funds, log

        # TODO: Add logging
        # TODO: Add flags
        # Take item/gold
        case "take":
            
            # Player has only typed regi take
            if len(args) == 0:
                return pretty_command("take", flags), bag, funds, log
            
            m_money = args[0]
            
            # Take money
            if is_valid_currency(m_money):
                money, currency = funds.parse_money_input(m_money)
                
                if funds.take_funds(money, currency):
                    return f"Took {money}{currency} out of our funds.", bag, funds, log
                else:
                    return f"We dont have enough {money}{currency} in our funds.", bag, funds, log
            # Take item
            
            # TODO: Implement id search, instead of just name
            item_name = args[0]
            amount = 1
            
            # Convert user input into int
            try:
                if len(args[1:]) >= 1:
                    amount = int(args[1])
            except ValueError:
                return f"I am sorry, I haven't heard of the number '{args[1]}'", bag, funds, log
            
            item = bag.get_item(item_name=item_name)
            
            # If item is none, the given item was not found
            if item is not None:
                if bag.remove_item(item, amount): # If remove_item returns true, the item was removed
                    return f"Took {amount} instances of {item}, out of our bag.", bag, funds, log
                else:
                    return f"Can't take {amount} instances of {item}, we only have {item.amount}", bag, funds, log
            else:
                return f"I am sorry, I can't find '{item_name}'", bag, funds, log
        
        # TODO: Add logging
        # TODO: Add flags
        # Find item
        case "find":
            
            # Player has only typed regi find
            if len(args) == 0:
                return pretty_command("find", flags), bag, funds, log
            
            item_name = args[0]
            amount = 1
            
            # Convert user input into int
            try:
                if len(args[1:]) >= 1:
                    amount = int(args[1])
            except ValueError:
                return f"I am sorry, I haven't heard of the number '{args[1]}'", bag, funds, log
            
            item = bag.get_item(item_name=item_name)
            
            if item is not None:
                return f"Found {item.item_name}, here's some information about it:\n{item.get_long_name()}", bag, funds, log
            else:
                return f"I am sorry, I can't find '{item_name}'", bag, funds, log
        
        # TODO: Add logging
        # TODO: Add flags
        # Displays funds
        case "funds":
            
            if len(args) == 0:
                return f"Here's our funds: {funds}", bag, funds, log
            
            c = args[1]
            
            if is_valid_currency(c):
                _, currency = funds.parse_money_input(f"0{c}")
                return f"Here's our funds in {str(currency).lower()}: {funds.funds_in(currency)}", bag, funds, log
            else:
                return f"I am sorry, I havent heard about the currency: '{args[1]}', but here's our funds in gold: {funds.funds_in(Currency.Gold)}", bag, funds, log
        
        # TODO: Add flags
        # TODO: Add logging
        # Displays the bag
        case "bag":
            return f"Here's our bag: {bag.get_all_items_short()}", bag, funds, log

        # TODO: Add flags
        # TODO: Add logging
        case "help":
            if len(args) == 0:
                return "\n".join([pretty_command(c, flags) for c in COMMANDS.keys()]), bag, funds, log
            
            msg = []
            
            for ar in args:
                if ar in COMMANDS.keys():
                    msg.append(pretty_command(ar, flags))
                elif ar in FLAGS.keys():
                    msg.append(pretty_flag(ar, flags))
                else:
                    msg.append(f"I have no knowledge about: '{ar}'")
            
            return "\n".join(msg), bag, funds, log
            
        case _:
            return f"I am sorry, I did not understand what you meant by '{p_message[1:]}', use 'help' to get all commands", bag, funds, log



def get_flags_and_arguments(arg: str) -> tuple[list[str], list[str]]:
    """Parses a string into a list of arguments, and a list of flags

    Args:
        arg (str): Argument string; the user input

    Returns:
        tuple[list[str], list[str]]: list of arguments, list of flags
    """
    
    flags = []
    
    args = []
    
    s = ""
    
    ant = 0
    
    # Iterates over each char in the str
    for c in arg:
        # If we hit one of these, its the start of and argument
        if c == '"' or c == "'":
            if ant == 1:
                args.append(s)
                ant = 0
                s = ""
        # If we hit space, and ant is not 0, it means we're inside an argument, and should take the space
        elif c == ' ' and ant != 0:
            s += c
        # If we hit a space, and ant is 0, it means we've hit the end of either an argument, or a flag
        elif c == ' ' and ant == 1:
            # Means its a flag
            if "--" in s[:2] or "-" in s[0]:
                flags.append(s)
                s = ""
            else: # its an argumetn
                args.append(s)
                s = ""
        # Else, we keep adding c to s
        else:
            s += c

    return args, flags


def pretty_flag(flag: str, flags: list[str]) -> str:
    """Returns a flag, as a pretty string, with info about the given flag.
    If the givne flag is not known, an empty string is returned instead.

    Args:
        flag (str): Flag
        flags (list[str]): Flags

    Returns:
        str: Flag, with info, ready for sending
    """
    if flag in FLAGS.keys():
        if "-example" in flags:
            return f"Example: `{FLAGS[flag]['example']}`\n"
        elif "-v" in flags:
            return f"Info: `{FLAGS[flag]['verbose info']}`\nExample: `{FLAGS[flag]['example']}`\n"
        else:
            return f"Info: `{FLAGS[flag]['info']}`\nExample: `{FLAGS[flag]['example']}`\n"
    return ""


def pretty_command(command: str, flags: list[str]) -> str:
    """Returns a command, as a pretty string, with info.
    If the given command is not known, an empty string is returned instead.

    Args:
        command (str): Command

    Returns:
        str: Command, with information, ready for sending.
    """
    if command in COMMANDS.keys():
        inf = COMMANDS[command]
        if "-example" in flags:
            return f"Example: `{inf['example']}`\n"
        else:
            return f"Command: `{command}`\nArguments: `{inf['args']}`\nExample: `{inf['example']}`\nFlags: `{inf['flags']}`\nInfo: `{inf['info']}`\n"
    return ""