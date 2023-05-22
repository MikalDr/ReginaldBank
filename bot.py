import discord
import responses
from bag_of_holding import BagOfHolding
from funds import Funds
import signal
import sys

def pre_termination_proc(signal, frame) -> None:
    """Saves the data in BAG and FUNDS, on termination"""
    global BAG, FUNDS
    print("Termination signal received. Performing cleanup...")
    BAG.save_items()
    FUNDS.save_funds()
    print("Cleanup done.")
    sys.exit(0)


# Ensures this function will be called on termination
signal.signal(signal.SIGINT, pre_termination_proc)
signal.signal(signal.SIGTERM, pre_termination_proc)

async def send_message(username, message, user_message, is_private):
    global BAG, FUNDS
    
    try:
        response, _BAG, _FUNDS = responses.handle_response(username, user_message, BAG, FUNDS)
        BAG = _BAG
        FUNDS = _FUNDS
        await message.author.send(response) if is_private else await message.channel.send(response)
        print("channel |",message.channel)
    except Exception as e:
        print(e)
        
BAG = BagOfHolding()    
BAG.load_items()

FUNDS = Funds()
FUNDS.load_funds()

def run_discord_bot():
    TOKEN = 'MTEwMjY4NDcxNDE3MjY3ODMwNQ.GOnDw3.xkiXUIRtLtqBQA7-XCVUWqhZisjo-PWku_XSIo'
    
    print("Hello1")
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