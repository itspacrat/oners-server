use {
    //liboners::*,
    oners_game::*,
    serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready, prelude::*},
        prelude::*,
    },
    std::{collections::HashMap, env, future::Future, path::Path},
    tokio::fs::{create_dir_all, read_to_string, write, OpenOptions},
    tokio::io::AsyncWriteExt,
    serde_json::{to_string_pretty,from_str},
};
pub mod oners_game;
struct Handler;
/// splits content into a vector of args by spaces
///
/// *TODO: quote marching*
fn collect_args<'a>(content: &'a String) -> Vec<&'a str> {
    let out = (*content).split(" ").map(|a| a).collect::<Vec<&str>>();
    out.clone()
}
const PREFIX: &str = "/one";
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
        let args = collect_args(&msg.content);

        if !(args[0] == PREFIX) {
            () // don't parse, not a command
        } else {
            println!("{:?}", &args[1..]);
            let id_c = &msg.channel_id.0;
            let id_u = &msg.author.id.0;
            let ping_out: String;
            //let ping_out: String;
            match args.get(1) {
                None => {
                    ping_out = format!("{}: missing subcommand", PREFIX);
                }
                Some(arg1) => {
                    match *arg1 {
                        "help" => {
                            ping_out = format!("h")
                            //future_replies.push(msg.reply(&ctx,format!("(help message)")));
                        }
                        "game" => match args.get(2) {
                            None => ping_out = format!("{}: missing argument", args[1]),
                            Some(arg2) => match *arg2 {
                                
                                "create" => {
                                    let id_c = &msg.channel_id.0;
                                    let id_u = &msg.author.id.0;
                                    let id_m = &msg
                                        .reply_ping(&ctx, "msg1")
                                        .await
                                        .unwrap()
                                        .to_owned()
                                        .id
                                        .0;
                                    let game_token = format!("{}.{}", &id_u, &id_m);

                                    create_dir_all(format!("games/{}", game_token))
                                        .await
                                        .unwrap();
                                    let _ = write(
                                        format!("games/{}/info.json", game_token),
                                        format!("{{\"players\":[{}]}}", &id_u).as_bytes(),
                                    )
                                    .await
                                    .unwrap();
                                    create_dir_all(format!("users/{}", &id_u)).await.unwrap();
                                    let _ = write(
                                        format!("users/{}/info.json", &id_u),
                                        format!("{{\"current_game\":\"{}\"}}", &game_token)
                                            .as_bytes(),
                                    )
                                    .await
                                    .unwrap();
                                    let _ = &ctx
                                        .http
                                        .get_message(*id_c, *id_m)
                                        .await
                                        .unwrap()
                                        .edit(&ctx, |m| m.content("game goes here"))
                                        .await
                                        .unwrap();
                                    ping_out = format!("game created.\ntoken:\n`{}`",game_token);
                                }
                                "join" => match args.get(3) {
                                    None => {
                                        ping_out = format!("missing game token");
                                    }
                                    Some(arg3) => {
                                        if !(Path::new(&format!("games/{}",arg3)).exists()) {
                                            ping_out = format!("game does not exist!"); 
                                        } else {
                                            let game_info: GameInfo = 
                                            from_str(
                                                read_to_string(format!("games/{}/info.json",arg3))
                                                .await.unwrap().as_str()
                                            ).unwrap();
                                            if game_info.players.contains(id_u) {
                                                ping_out = format!("you are already a part of this game!");
                                            } else {
                                                ping_out = format!("looks like you can join!");
                                            }
                                        }
                                        
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
                    }
                }
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
