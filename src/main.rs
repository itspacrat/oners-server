use {
    serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    },
    std::{collections::HashMap, env, path::Path},
    liboners::*,
    tokio::fs::{create_dir_all, read_to_string, write, OpenOptions},
    tokio::io::AsyncWriteExt,
};
struct Handler;
/// splits content into args by spaces
/// TODO: quote marching
fn collect_args<'a>(content:&'a String) -> Vec<&'a str>{
    let out = (*content).split(" ").map(|a|a).collect::<Vec<&str>>();
    out.clone()
}

#[async_trait]
impl EventHandler for Handler {
    
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        
        let mut ping_out: String = String::from("");
        match msg.content.as_str() {
            "!ping" => {
                ping_out = String::from("hello!!!!!!");
                let _ = msg.reply_ping(ctx, format!("{}", ping_out)).await.unwrap();
            }
            _ => {}
        }
        println!("{}", ping_out);
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
