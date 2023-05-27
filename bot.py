import discord
import responses
import commands
from bag_of_holding import BagOfHolding
from funds import Funds
from logger import Log
from sessions import SessionManager
import signal
import sys

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
    global BAG, FUNDS, LOG, SM
    
    try:
        #response, _BAG, _FUNDS, _LOG = responses.handle_response(username, user_message, BAG, FUNDS, LOG)
        response, _BAG, _FUNDS, _LOG, _SM = commands.parse_command(username, user_message, BAG, FUNDS, LOG, SM)
        BAG = _BAG
        FUNDS = _FUNDS
        LOG = _LOG
        SM = _SM
        
        if not response:
            return
        
        await message.author.send(response) if is_private else await message.channel.send(response)
        print("channel |",message.channel)
    except Exception as e:
        print(f"Exception:\n{e}\nStacktrace:{e.with_traceback()}\n")
        
BAG = BagOfHolding()    
BAG.load_items()

FUNDS = Funds()
FUNDS.load_funds()

LOG = Log()
LOG.load_log()

SM = SessionManager()

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