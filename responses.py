call_names = set(["reginald","regi", "rabbit", "hare"])
player_tags = {"4993":"Zenith", "0492":"Scorch", "9933":"me", "2418":"Kerke", "8705":"Halfdan", "9521":"Marxson", "8054": "Strange Godlike Being"}
scorch_greetings = ["Please be careful today scorch, i do not like my hair scorched...", "Scorch! stay away with that blasted fire!", "Good Morning Scorch!"]

def handle_response(username, message) -> str:
    tag = username.split("#")[1]
    name = player_tags.get(tag)
    
    