call_names = set(["reginald","regi", "rabbit", "hare"])
player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}
scorch_greetings = ["Please be careful today scorch, i do not like my hair scorched...", "Scorch! stay away with that blasted fire!", "Good Morning Scorch!"]

def handle_response(username, message) -> str:
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    p_message = message.lower().split(" ")
    
    if p_message[0] in call_names:
        match p_message[1]:
            # Money
            case "funds":
                # If the length is two, return the funds
                if(len(p_message) == 2):
                    pass # Return the funds
                match p_message[2]:
                    case "in":
                        pass # Returns the funds in chosen currency
                    case "add":
                        pass # Adds to the funds
                    case "take":
                        pass # Removes from the funds
                        
                pass
            # Bag of holding
            case "bag":
                pass
            

        return "Hello"