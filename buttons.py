import discord
from discord import ActionRow
from funds import Funds, is_valid_currency
from funds import Currency
from bag_of_holding import BagOfHolding

class MainView(discord.ui.View):

    global FUNDS, BOH

    FUNDS = Funds()
    FUNDS.load_funds()
    BOH = BagOfHolding()
    BOH.load_items()

    @discord.ui.button(label="Funds", row=0, style=discord.ButtonStyle.primary, emoji="💰")
    async def funds_button_callback(self, button, interaction):
        #interaction.interaction.message.channel_id.send("You clicked the button!")
        await interaction.response.send_message(f"Hi {interaction.user} here's what we've got! ```{FUNDS.__str__()} \n\nTotal in gold: {FUNDS.funds_in(Currency.Gold)}```", view=FundsView(), delete_after=30.0)

    @discord.ui.button(label="BoH", row=0, style=discord.ButtonStyle.primary, emoji="💼")
    async def boh_button_callback(self, button, interaction):
        await interaction.response.send_message(f"Hi {interaction.user}, Here is our bag: ```Items:\n{BOH.get_all_items_short()}\n\nTotal Value: {BOH.get_total_worth()} \nTotal Weigth: {BOH.get_total_weight()}```", view=BoHView(), delete_after=30.0)
    
    @discord.ui.button(label="Logs", row=0, style=discord.ButtonStyle.primary, emoji="📜")
    async def log_button_callback(self, button, interaction):
        await interaction.response.send_message(f"Sorry {interaction.user}, Logs are not yet implemented.", delete_after=30.0)

class BoHView(discord.ui.View):
    @discord.ui.button(label="Add", row=0, style=discord.ButtonStyle.primary, emoji="➕")
    async def add_button_callback(self, button, interaction):
        await interaction.response.send_modal(AddItemModal(title="Add Item"))
    @discord.ui.button(label="Remove", row=0, style=discord.ButtonStyle.primary, emoji="➖")
    async def take_button_callback(self, button, interaction):
        await interaction.response.send_modal(TakeItemModal(title="Take Item"))

class LogView(discord.ui.View):
    pass

class FundsView(discord.ui.View):
    @discord.ui.button(label="Add", row=0, style=discord.ButtonStyle.primary, emoji="➕")
    async def add_button_callback(self, button, interaction):
        await interaction.response.send_modal(AddModal(title="Add Item"))
    @discord.ui.button(label="Take", row=0, style=discord.ButtonStyle.primary, emoji="➖")
    async def take_button_callback(self, button, interaction):
        await interaction.response.send_modal(TakeModal(title="Take Item"))

@discord.slash_command()
async def send_modal(ctx):
    await ctx.respond(view=(FundsView()))

class AddItemModal(discord.ui.Modal):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.add_item(discord.ui.InputText(label="Quantity"))
        self.add_item(discord.ui.InputText(label="Item"))

    async def callback(self, interaction):
        embed = discord.Embed(title="Added")
        await interaction.response.send_message("Function not yet added", delete_after=10.0)

class TakeItemModal(discord.ui.Modal):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.add_item(discord.ui.InputText(label="Quantity"))
        self.add_item(discord.ui.InputText(label="Item"))

    async def callback(self, interaction):
        embed = discord.Embed(title="Added")
        await interaction.response.send_message("Function not yet added", delete_after=10.0)

class AddModal(discord.ui.Modal):

    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.add_item(discord.ui.InputText(label="Amount"))

    async def callback(self, interaction):
        embed = discord.Embed(title="Added")

        if is_valid_currency("".join([c for c in self.children[0].value if not c.isdigit()])):
            # Add money
            money, currency = FUNDS.parse_money_input(self.children[0].value)
            
            FUNDS.add_funds(money, currency)
            print(money,currency)
            embed.add_field(name="Amount", value=f"{money} {currency.__str__()}")
            await interaction.response.send_message(f"{interaction.user} Added {money} {currency.__str__()},\nhere's what we've now got! ```{FUNDS.__str__()} \n\nTotal in gold: {FUNDS.funds_in(Currency.Gold)}```", delete_after=30.0)
        else:
           await interaction.response.send_message(f"{interaction.user}, that is not a valid currency", delete_after=30.0)

class TakeModal(discord.ui.Modal):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.add_item(discord.ui.InputText(label="Amount"))

    async def callback(self, interaction):
        embed = discord.Embed(title="Took")

        if is_valid_currency("".join([c for c in self.children[0].value if not c.isdigit()])):
            # Add money
            money, currency = FUNDS.parse_money_input(self.children[0].value)
            
            FUNDS.take_funds(money, currency)
            print(money,currency)
            embed.add_field(name="Amount", value=f"{money} {currency.__str__()}")
            await interaction.response.send_message(f"{interaction.user} Took {money} {currency.__str__()},\nhere's what we've now got! ```{FUNDS.__str__()} \n\nTotal in gold: {FUNDS.funds_in(Currency.Gold)}```", delete_after=30.0)

        else:
           await interaction.response.send_message(f"{interaction.user}, that is not a valid currency", delete_after=30.0)