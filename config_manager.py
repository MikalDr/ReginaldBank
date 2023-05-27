"""
Manages the config files for the discord bot
"""
from typing import Any, Optional
import json

FILE = "files/config.json"

CALL_NAME_KEY = "call-name"

DEFAULT_CALL_NAME = ["reginald","regi", "rabbit", "hare"]

GREETING_KEY = "greetings"

DEFAULT_GREETING = ["Greetings!", "Hey!", "Hi!", "Hello!"]

PLAYER_KEY = "players"
PLAYER_NICKNAME_KEY = "nickname"

COMMAND_ALIAS_KEY = "command-alias"

FLAG_ALIAS_KEY = "flag-alias"

COMMAND_DESCRIPTION_KEY = "command-description"

CMD_INFO_KEY = "info"
CMD_FLAG_KEY = "flags"

DEFAULT_CONFIG = {
    CALL_NAME_KEY: DEFAULT_CALL_NAME,
    PLAYER_KEY: [

    ],
    COMMAND_ALIAS_KEY: {
        
    },
    FLAG_ALIAS_KEY: {

    },
    COMMAND_DESCRIPTION_KEY: {
        cmd : {
            CMD_INFO_KEY : info,
            CMD_FLAG_KEY : flag
        } for cmd, info, flag in COMMANDS
    }
}

class ConfigManager:
    def __init__(self, config: Optional[dict[str, Any]]):
        if config is None:
            config = {}
        self.config = config
    
    def validate_config(self, config: dict[str, Any]) -> bool:
        raise NotImplementedError
        
    def get_callname(self) -> list[str]:
        return self.config[CALL_NAME_KEY] if CALL_NAME_KEY in self.config.keys() else DEFAULT_CALL_NAME

    def get_greetings(self, user: str) -> list[str]:
        raise NotImplementedError(user)

    def init_config(self) -> None:
        with open(FILE, "w", encoding="utf-8") as f:
            json_object = json.dumps(DEFAULT_CONFIG, indent=4)
            f.write(json_object)
            print(f"Initialized config.json file at: {FILE}")
        
    def add_users_to_config(self, users: list[str]) -> None:
        if self.config[PLAYER_KEY] is None:
            self.config[PLAYER_KEY] = []
        for user in users:
            self.config[PLAYER_KEY].append({user: {GREETING_KEY: [], PLAYER_NICKNAME_KEY: []}})