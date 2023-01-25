use {
    //liboners::*,
    oners_game::*,
    serde_json::{from_str, to_string_pretty},
    serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready, prelude::*},
        prelude::*,
    },
    std::{collections::HashMap, env, future::Future, path::Path},
    tokio::fs::{create_dir_all, read_to_string, write, OpenOptions},
    tokio::io::AsyncWriteExt,
};
pub mod oners_game;
struct Handler;

const PREFIX: &str = ".o";
#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `add reaction` event - so that whenever a new reaction
    // is added - the closure (or function) passed will be called.
    async fn reaction_add(&self, ctx: Context, re: Reaction) {
        println!("detected reaction {:?}", &re);
    }
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, mut msg: Message) {
        let args = (&msg.content).split(" ").map(|a| a).collect::<Vec<&str>>();

        if !(args[0] == PREFIX) {
            () // don't parse, not a command
        } else {
            println!("{:?}", &args[1..]);
            let id_c = &msg.channel_id.0;
            let id_u = &msg.author.id.0;
            let ping_out: String;

            match args.get(1) {
                None => {
                    ping_out = format!("{}: missing subcommand", PREFIX);
                }
                Some(arg1) => match *arg1 {
                    "help" => ping_out = format!("h"),
                    "game" => match args.get(2) {
                        None => ping_out = format!("{}: missing argument", args[1]),
                        Some(arg2) => match *arg2 {
                            "create" => {
                                ping_out = create_game(&ctx, &msg).await;
                            }
                            "join" => match args.get(3) {
                                None => {
                                    ping_out = format!("missing game token");
                                }
                                Some(arg3) => {
                                    ping_out = join_game(id_u, arg3).await;
                                }
                            },
                            _ => {
                                ping_out = format!("game: not a valid argument");
                            }
                        },
                    },
                    _ => {
                        ping_out = format!("no command: \"{}\"", args[1]);
                    }
                },
            }
            let bot_msg_id = msg
                .reply_ping(ctx, format!("{}", ping_out))
                .await
                .unwrap()
                .id
                .0;
            println!("{} -> {}", bot_msg_id, ping_out);
        }
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
