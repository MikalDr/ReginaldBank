import discord
import responses
import commands
from bag_of_holding import BagOfHolding
from funds import Funds
from logger import Log
from buttons import MainView
import signal
import sys
from config import Config

TOKEN = open("files/token.txt").read()

def pre_termination_proc(signal, frame) -> None:
    """Saves the data in BAG and FUNDS, on termination"""
    global BAG, FUNDS, LOG
    print("Termination signal received. Performing cleanup...")
    #BAG.save_items()
    FUNDS.save_funds()
    LOG.save_log()
    print("Cleanup done.")
    sys.exit(0)


# Ensures this function will be called on termination
signal.signal(signal.SIGINT, pre_termination_proc)
signal.signal(signal.SIGTERM, pre_termination_proc)

async def send_message(username, message, user_message, is_private):

    global BAG, FUNDS, LOG, CONFIG
    
    try:
        #response, _BAG, _FUNDS, _LOG = responses.handle_response(username, user_message, BAG, FUNDS, LOG)
        response, _BAG, _FUNDS, _LOG = commands.parse_command(username, user_message, BAG, FUNDS, LOG)
        BAG = _BAG
        FUNDS = _FUNDS
        LOG = _LOG
        if(user_message == "regi init"):
            CONFIG.add_channel(message.channel.name)
            await message.channel.send(f'Greetings! I am Reginald-Bot. \nI will try to assist you with your needs.\nTo start a conversation with me, please type "hi regi"')
            await message.delete()
        if(user_message == "hi regi"):
            await message.channel.send(f"Hi {username}! what do you wish for?", view=MainView(), delete_after=240.0)
            await message.delete()
        print("channel |",message.channel)
    except Exception as e:
        print(e)

CONFIG = Config()
CONFIG.load_config()

BAG = BagOfHolding()    
BAG.load_items()

FUNDS = Funds()
FUNDS.load_funds()

LOG = Log()
LOG.load_log()

def run_discord_bot():
    intents = discord.Intents.default()
    intents.message_content = True
    client = discord.Client(intents=intents)
    
    
    @client.event
    async def on_ready():
        print(f'{client.user} is now running!')
        
    @client.event
    async def on_message(message):
        if message.author == client.user:
            return
        username = str(message.author)
        user_message = str(message.content)
        channel = str(message.channel)
        
        print(f"{username} said: '{user_message}', {channel}")
        
        if user_message[0] == '?':
            user_message = user_message[1:]
            await send_message(username, message, user_message, is_private=True)
        else:
            await send_message(username, message, user_message, is_private=False)
        
    client.run(TOKEN)