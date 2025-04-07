// use crate::api::fetch_with_backoff;
// use crate::models::Coin;
// use serenity::async_trait;
// use serenity::model::channel::Message;
// use serenity::model::gateway::Ready;
// use serenity::model::id::ChannelId;
// use serenity::prelude::*;
// use log::{debug, error, info};
// use std::env;
// use std::sync::Arc;
// use std::time::Duration;
// use tokio::sync::RwLock;

// pub struct Handler {
//     channel_id: Arc<RwLock<Option<ChannelId>>>,
//     last_coins: Arc<RwLock<Vec<Coin>>>,
// }

// impl Handler {
//     pub fn new(channel_id: Arc<RwLock<Option<ChannelId>>>) -> Self {
//         info!("Handler created with channel_id Arc: {:?}", Arc::as_ptr(&channel_id));
//         Self {
//             channel_id,
//             last_coins: Arc::new(RwLock::new(Vec::new())),
//         }
//     }

//     async fn send_coin_message(&self, ctx: &Context, channel_id: ChannelId, coin: &Coin, prefix: &str) {
//         let msg = format!(
//             "{} Zora Coin!\nName: {}\nSymbol: {}\nImage: {}\nCreated: {}",
//             prefix, coin.name, coin.symbol, coin.media_content.preview_image.small, coin.created_at
//         );
//         if let Err(e) = channel_id.say(&ctx.http, &msg).await {
//             error!("Failed to send message: {}", e);
//         } else {
//             info!("Sent message to channel {}: {}", channel_id, msg);
//         }
//     }
// }

// #[async_trait]
// impl EventHandler for Handler {
//     async fn ready(&self, ctx: Context, ready: Ready) {
//         info!("Bot is ready as {}", ready.user.name);

//         let new_coins_interval = env::var("NEW_COINS_INTERVAL_SECONDS")
//             .unwrap_or("60".to_string())
//             .parse::<u64>()
//             .unwrap_or(60);
//         let top_gainers_interval = env::var("TOP_GAINERS_INTERVAL_SECONDS")
//             .unwrap_or("300".to_string())
//             .parse::<u64>()
//             .unwrap_or(300);

//         let ctx_clone = ctx.clone();
//         let channel_id_clone = self.channel_id.clone();
//         let last_coins_clone = self.last_coins.clone();
//         tokio::spawn(async move {
//             loop {
//                 if let Some(channel_id) = *channel_id_clone.read().await {
//                     match fetch_with_backoff("NEW", 3).await {
//                         Ok(coins) => {
//                             let mut last_coins = last_coins_clone.write().await;
//                             let new_coins: Vec<_> = coins
//                                 .iter()
//                                 .filter(|c| !last_coins.iter().any(|lc| lc.address == c.address))
//                                 .collect();
//                             for coin in new_coins {
//                                 Handler::new(channel_id_clone.clone())
//                                     .send_coin_message(&ctx_clone, channel_id, coin, "New")
//                                     .await;
//                             }
//                             *last_coins = coins;
//                         }
//                         Err(e) => error!("Error fetching NEW coins: {}", e),
//                     }
//                 } else {
//                     debug!("Channel ID not set, skipping new coins update");
//                 }
//                 tokio::time::sleep(Duration::from_secs(new_coins_interval)).await;
//             }
//         });

//         let ctx_clone = ctx.clone();
//         let channel_id_clone = self.channel_id.clone();
//         tokio::spawn(async move {
//             loop {
//                 if let Some(channel_id) = *channel_id_clone.read().await {
//                     match fetch_with_backoff("TOP_GAINERS", 3).await {
//                         Ok(coins) if !coins.is_empty() => {
//                             let top_coin = &coins[0];
//                             let msg = format!(
//                                 "Top Gainer: {}\nSymbol: {}\n24h Gain: ${}\nImage: {}\nTrades: {}",
//                                 top_coin.name, top_coin.symbol, top_coin.market_cap_delta_24h,
//                                 top_coin.media_content.preview_image.small, top_coin.transfers.count
//                             );
//                             if let Err(e) = channel_id.say(&ctx_clone.http, &msg).await {
//                                 error!("Failed to send top gainer message: {}", e);
//                             } else {
//                                 info!("Sent top gainer message to channel {}: {}", channel_id, msg);
//                             }
//                         }
//                         Ok(_) => debug!("No top gainers returned"),
//                         Err(e) => error!("Error fetching TOP_GAINERS: {}", e),
//                     }
//                 } else {
//                     debug!("Channel ID not set, skipping top gainers update");
//                 }
//                 tokio::time::sleep(Duration::from_secs(top_gainers_interval)).await;
//             }
//         });
//     }

//     async fn message(&self, ctx: Context, msg: Message) {
//         debug!("Received message: {}", msg.content);

//         if msg.author.bot {
//             return;
//         }

//         let content = msg.content.trim();
//         let channel_id = *self.channel_id.read().await;

//         match content {
//             "!fetch_new_coins" => {
//                 if let Some(channel_id) = channel_id {
//                     match fetch_with_backoff("NEW", 3).await {
//                         Ok(coins) => {
//                             for coin in coins {
//                                 self.send_coin_message(&ctx, channel_id, &coin, "New").await;
//                             }
//                         }
//                         Err(e) => {
//                             let _ = msg.reply(&ctx.http, format!("Error fetching new coins: {}", e)).await;
//                         }
//                     }
//                 } else {
//                 let _ = msg.reply(&ctx.http, "Channel not set. Use !notify_this_channel or the HTTP route to set it.").await;
//                 }
//             }
//             "!fetch_top_gainers" => {
//                 if let Some(channel_id) = channel_id {
//                     match fetch_with_backoff("TOP_GAINERS", 3).await {
//                         Ok(coins) if !coins.is_empty() => {
//                             let top_coin = &coins[0];
//                             let msg = format!(
//                                 "Top Gainer: {}\nSymbol: {}\n24h Gain: ${}\nImage: {}\nTrades: {}",
//                                 top_coin.name, top_coin.symbol, top_coin.market_cap_delta_24h,
//                                 top_coin.media_content.preview_image.small, top_coin.transfers.count
//                             );
//                             if let Err(e) = channel_id.say(&ctx.http, &msg).await {
//                                 error!("Failed to send top gainer message: {}", e);
//                             }
//                         }
//                         Ok(_) => {
//                             let _ = msg.reply(&ctx.http, "No top gainers found.").await;
//                         }
//                         Err(e) => {
//                             let _ = msg.reply(&ctx.http, format!("Error fetching top gainers: {}", e)).await;
//                         }
//                     }
//                 } else {
//                     let _ = msg.reply(&ctx.http, "Channel not set. Use !notify_this_channel or the HTTP route to set it.").await;
//                 }
//             }
//             "!check_channel" => {
//                 debug!("Processing !check_channel command");
//                 let response = if let Some(id) = channel_id {
//                     format!("Current channel ID: {}", id)
//                 } else {
//                     "Channel ID not set".to_string()
//                 };
//                 if let Err(e) = msg.reply(&ctx.http, &response).await {
//                     error!("Failed to reply to !check_channel: {}", e);
//                 } else {
//                     info!("Replied to !check_channel with: {}", response);
//                 }
//             }
//             "!notify_this_channel" => {
//                 debug!("Processing !notify_this_channel command");
//                 let new_channel_id = msg.channel_id;
//                 *self.channel_id.write().await = Some(new_channel_id);
//                 let response = format!("Notifications will now be sent to this channel (ID: {})", new_channel_id);
//                 if let Err(e) = msg.reply(&ctx.http, &response).await {
//                     error!("Failed to reply to !notify_this_channel: {}", e);
//                 } else {
//                     info!("Channel ID set to {} via !notify_this_channel", new_channel_id);
//                 }
//             }
//             _ => {}
//         }
//     }
// }

use crate::api::{fetch_with_backoff, create_zora_token};
use crate::models::Coin;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use log::{debug, error, info};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct Handler {
    channel_id: Arc<RwLock<Option<ChannelId>>>,
    last_coins: Arc<RwLock<Vec<Coin>>>,
}

impl Handler {
    pub fn new(channel_id: Arc<RwLock<Option<ChannelId>>>) -> Self {
        info!("Handler created with channel_id Arc: {:?}", Arc::as_ptr(&channel_id));
        Self {
            channel_id,
            last_coins: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn send_coin_message(&self, ctx: &Context, channel_id: ChannelId, coin: &Coin, prefix: &str) {
        let msg = format!(
            "{} Zora Coin!\nName: {}\nSymbol: {}\nImage: {}\nCreated: {}",
            prefix, coin.name, coin.symbol, coin.media_content.preview_image.small, coin.created_at
        );
        if let Err(e) = channel_id.say(&ctx.http, &msg).await {
            error!("Failed to send message: {}", e);
        } else {
            info!("Sent message to channel {}: {}", channel_id, msg);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Bot is ready as {}", ready.user.name);

        let new_coins_interval = env::var("NEW_COINS_INTERVAL_SECONDS")
            .unwrap_or("60".to_string())
            .parse::<u64>()
            .unwrap_or(60);
        let top_gainers_interval = env::var("TOP_GAINERS_INTERVAL_SECONDS")
            .unwrap_or("300".to_string())
            .parse::<u64>()
            .unwrap_or(300);

        let ctx_clone = ctx.clone();
        let channel_id_clone = self.channel_id.clone();
        let last_coins_clone = self.last_coins.clone();
        tokio::spawn(async move {
            loop {
                if let Some(channel_id) = *channel_id_clone.read().await {
                    match fetch_with_backoff("NEW", 3).await {
                        Ok(coins) => {
                            let mut last_coins = last_coins_clone.write().await;
                            let new_coins: Vec<_> = coins
                                .iter()
                                .filter(|c| !last_coins.iter().any(|lc| lc.address == c.address))
                                .collect();
                            for coin in new_coins {
                                Handler::new(channel_id_clone.clone())
                                    .send_coin_message(&ctx_clone, channel_id, coin, "New")
                                    .await;
                            }
                            *last_coins = coins;
                        }
                        Err(e) => error!("Error fetching NEW coins: {}", e),
                    }
                } else {
                    debug!("Channel ID not set, skipping new coins update");
                }
                tokio::time::sleep(Duration::from_secs(new_coins_interval)).await;
            }
        });

        let ctx_clone = ctx.clone();
        let channel_id_clone = self.channel_id.clone();
        tokio::spawn(async move {
            loop {
                if let Some(channel_id) = *channel_id_clone.read().await {
                    match fetch_with_backoff("TOP_GAINERS", 3).await {
                        Ok(coins) if !coins.is_empty() => {
                            let top_coin = &coins[0];
                            let msg = format!(
                                "Top Gainer: {}\nSymbol: {}\n24h Gain: ${}\nImage: {}\nTrades: {}",
                                top_coin.name, top_coin.symbol, top_coin.market_cap_delta_24h,
                                top_coin.media_content.preview_image.small, top_coin.transfers.count
                            );
                            if let Err(e) = channel_id.say(&ctx_clone.http, &msg).await {
                                error!("Failed to send top gainer message: {}", e);
                            } else {
                                info!("Sent top gainer message to channel {}: {}", channel_id, msg);
                            }
                        }
                        Ok(_) => debug!("No top gainers returned"),
                        Err(e) => error!("Error fetching TOP_GAINERS: {}", e),
                    }
                } else {
                    debug!("Channel ID not set, skipping top gainers update");
                }
                tokio::time::sleep(Duration::from_secs(top_gainers_interval)).await;
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        debug!("Received message: {}", msg.content);

        if msg.author.bot {
            return;
        }

        let content = msg.content.trim();
        let channel_id = *self.channel_id.read().await;

        match content {
            "!fetch_new_coins" => {
                if let Some(channel_id) = channel_id {
                    match fetch_with_backoff("NEW", 3).await {
                        Ok(coins) => {
                            for coin in coins {
                                self.send_coin_message(&ctx, channel_id, &coin, "New").await;
                            }
                        }
                        Err(e) => {
                            let _ = msg.reply(&ctx.http, format!("Error fetching new coins: {}", e)).await;
                        }
                    }
                } else {
                    let _ = msg.reply(&ctx.http, "Channel not set. Use !notify_this_channel or the HTTP route to set it.").await;
                }
            }
            "!fetch_top_gainers" => {
                if let Some(channel_id) = channel_id {
                    match fetch_with_backoff("TOP_GAINERS", 3).await {
                        Ok(coins) if !coins.is_empty() => {
                            let top_coin = &coins[0];
                            let msg = format!(
                                "Top Gainer: {}\nSymbol: {}\n24h Gain: ${}\nImage: {}\nTrades: {}",
                                top_coin.name, top_coin.symbol, top_coin.market_cap_delta_24h,
                                top_coin.media_content.preview_image.small, top_coin.transfers.count
                            );
                            if let Err(e) = channel_id.say(&ctx.http, &msg).await {
                                error!("Failed to send top gainer message: {}", e);
                            }
                        }
                        Ok(_) => {
                            let _ = msg.reply(&ctx.http, "No top gainers found.").await;
                        }
                        Err(e) => {
                            let _ = msg.reply(&ctx.http, format!("Error fetching top gainers: {}", e)).await;
                        }
                    }
                } else {
                    let _ = msg.reply(&ctx.http, "Channel not set. Use !notify_this_channel or the HTTP route to set it.").await;
                }
            }
            "!check_channel" => {
                debug!("Processing !check_channel command");
                let response = if let Some(id) = channel_id {
                    format!("Current channel ID: {}", id)
                } else {
                    "Channel ID not set".to_string()
                };
                if let Err(e) = msg.reply(&ctx.http, &response).await {
                    error!("Failed to reply to !check_channel: {}", e);
                } else {
                    info!("Replied to !check_channel with: {}", response);
                }
            }
            "!notify_this_channel" => {
                debug!("Processing !notify_this_channel command");
                let new_channel_id = msg.channel_id;
                *self.channel_id.write().await = Some(new_channel_id);
                let response = format!("Notifications will now be sent to this channel (ID: {})", new_channel_id);
                if let Err(e) = msg.reply(&ctx.http, &response).await {
                    error!("Failed to reply to !notify_this_channel: {}", e);
                } else {
                    info!("Channel ID set to {} via !notify_this_channel", new_channel_id);
                }
            }
            // _ if content.starts_with("!create_token") => {
            //     let args: Vec<&str> = content.split_whitespace().collect();
            //     if args.len() < 4 {
            //         let _ = msg.reply(&ctx.http, "Usage: !create_token <name> <symbol> <metadata>").await;
            //         return;
            //     }

            //     let name = args[1];
            //     let symbol = args[2];
            //     let metadata = args[3];

            //     match create_zora_token(name, symbol, metadata).await {
            //         Ok(response) => {
            //             let _ = msg.reply(&ctx.http, format!("Token created successfully: {}", response)).await;
            //         }
            //         Err(error) => {
            //             let _ = msg.reply(&ctx.http, format!("Failed to create token: {}", error)).await;
            //         }
            //     }
            // }
            // _ if content.starts_with("!create_token") {
            //     let args: Vec<&str> = content.split_whitespace().collect();
            //     if args.len() < 5 {
            //         let _ = msg.reply(&ctx.http, "Usage: !create_token <name> <symbol> <uri> <payout_recipient>").await;
            //         return;
            //     }
            //
            //     let name = args[1].to_string();
            //     let symbol = args[2].to_string();
            //     let uri = args[3].to_string();
            //     let payout_recipient: Address = args[4].parse().expect("Invalid Ethereum address");
            //
            //     let params = CoinParams {
            //         name,
            //         symbol,
            //         uri,
            //         payout_recipient,
            //         platform_referrer: None,
            //         initial_purchase_wei: U256::zero(),
            //     };
            //
            //     match create_coin(params).await {
            //         Ok(receipt) => {
            //             let _ = msg.reply(&ctx.http, format!("Token created successfully! Transaction hash: {:?}", receipt.transaction_hash)).await;
            //         }
            //         Err(error) => {
            //             let _ = msg.reply(&ctx.http, format!("Failed to create token: {}", error)).await;
            //         }
            //     }
            // }
            _ => {}
        }
    }
}