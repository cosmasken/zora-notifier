NOTE : Render's free instance will spin down with inactivity, which can delay requests by 50 seconds or more.

# Zora Notify Bot

Zora Notify Bot is a Discord bot that interacts with the Zora API and Ethereum blockchain to notify users about new coins, top gainers, and even create new tokens directly from Discord.

## Features
- **Fetch New Coins**: Periodically fetches newly minted coins from the Zora API and sends notifications to a Discord channel.
- **Fetch Top Gainers**: Periodically fetches the top-gaining coins and sends notifications.
- **Create Tokens**: Allows authorized users to create new tokens on the Ethereum blockchain via Discord commands.
- **Dynamic Channel Configuration**: Set the notification channel dynamically using commands or an HTTP route.

---

## Prerequisites
1. **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. **Discord Bot Token**: Create a bot on the [Discord Developer Portal](https://discord.com/developers/applications) and copy the bot token.
3. **Ethereum RPC URL**: Use a service like [Infura](https://infura.io/) or [Alchemy](https://www.alchemy.com/) to get an Ethereum RPC URL.
4. **Private Key**: Obtain the private key of the Ethereum wallet you want to use for deploying tokens.

---

## Installation

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/zora-notifier.git
cd zora-notifier
```
### 2. Set Up Environment Variables
Create a `.env` file in the root directory and add the following:

```env
DISCORD_BOT_TOKEN=your-discord-bot-token
ETHEREUM_RPC_URL=your-ethereum-rpc-url
PRIVATE_KEY=your-private-key
```

### 3. Install Dependencies
Run the following command to install the required dependencies:

```bash
cargo build
```

### Usage

#### 1. Start the Bot
Run the bot using:

```bash
cargo run
```

#### 2. Invite the Bot to Your Server
1. Go to the [Discord Developer Portal](https://discord.com/developers/applications).
2. Select your bot, go to the "OAuth2" tab, and generate an invite link with the required permissions.
3. Invite the bot to your server using the generated link.
### Commands

#### 1. Set Notification Channel
Set the current channel as the notification channel:
```bash
!set_channel
```

#### 2. Fetch New Coins
Manually fetch and display newly minted coins:
```bash
!fetch_new_coins
```

#### 3. Fetch Top Gainers
Manually fetch and display the top-gaining coins:
```bash
!fetch_top_gainers
```

#### 4. Create a New Token
Create a new token on the Ethereum blockchain:
```bash
!create_token <name> <symbol> <uri> <payout_recipient>
```
- **name**: The name of the token.
- **symbol**: The symbol of the token.
- **uri**: The metadata URI (e.g., IPFS link).
- **payout_recipient**: The Ethereum address to receive payouts.

Example:
```bash
!create_token MyToken MYT ipfs://example-uri 0x1234567890abcdef1234567890abcdef12345678
```

---

### HTTP API

#### Set Notification Channel
You can also set the notification channel via an HTTP POST request:
```http
POST /set_channel
Content-Type: application/json

{
    "channel_id": "123456789012345678"
}
```
Replace `123456789012345678` with the desired channel ID.

---

### Access Control
To restrict access to certain commands (e.g., `!create_token`), you can:
- Restrict by user ID or role.
- Add a confirmation step before executing sensitive commands.

---

### Troubleshooting

#### 1. Bot Not Responding
- Ensure the bot is running and connected to Discord.
- Check if the bot has the necessary permissions in the channel.

#### 2. Failed to Create Token
- Verify that your Ethereum RPC URL and private key are correct.
- Ensure the wallet has enough ETH to cover gas fees.

#### 3. API Errors
- Check if the Zora API key is valid and has the required permissions.
- Ensure the Zora API endpoint is reachable.

---

### Contributing
Feel free to submit issues or pull requests to improve the bot.

---

### License
This project is licensed under the MIT License.
