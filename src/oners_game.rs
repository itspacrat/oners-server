use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum OnersState {
    Init,
    Start,
    Turn,
    Draw,
    End,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GameInfo {
    pub players: Vec<u64>,
    pub state: OnersState,
}
pub async fn subcommand_game() -> String {
    format!("")
}
pub async fn create_game(ctx: &Context, msg: &Message) -> String {
    let (id_c, id_u, id_m) = (
        msg.channel_id.0,
        msg.author.id.0,
        msg.reply_ping(ctx, "msg1").await.unwrap().to_owned().id.0,
    );
    let game_token = format!("{}.{}", &id_u, &id_m);

    create_dir_all(format!("games/{}", game_token))
        .await
        .unwrap();
    let _ = write(
        format!("games/{}/info.json", game_token),
        format!("{{\"players\":[{}],\"state\":\"Init\"}}", &id_u).as_bytes(),
    )
    .await
    .unwrap();
    create_dir_all(format!("users/{}", &id_u)).await.unwrap();
    let _ = write(
        format!("users/{}/info.json", &id_u),
        format!("{{\"current_game\":\"{}\"}}", &game_token).as_bytes(),
    )
    .await
    .unwrap();
    let _ = &ctx
        .http
        .get_message(id_c, id_m)
        .await
        .unwrap()
        .edit(&ctx, |m| m.content("game goes here"))
        .await
        .unwrap();
    format!("game created.\ntoken:\n`{}`", game_token)
}

// uses the user's discord id and a game token to join an existing game
pub async fn join_game(user_id: &u64, game_token: &str) -> String {
    if !(Path::new(&format!("games/{}", game_token)).exists()) {
        format!("game does not exist: {}", game_token)
    } else {
        let mut game_info: GameInfo = from_str(
            read_to_string(format!("games/{}/info.json", game_token))
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap();
        if game_info.players.contains(user_id) {
            format!("you are already a part of this game!")
        } else {
            game_info.players.push(*user_id);
            let _ = write(
                format!("games/{}/info.json", game_token),
                to_string_pretty(&game_info).unwrap(),
            )
            .await
            .unwrap();
            let _ = {
                create_dir_all(format!("users/{}", user_id)).await.unwrap();
                write(
                    format!("users/{}/info.json", user_id),
                    format!("{{\"current_game\":\"{}\"}}", &game_token).as_bytes(),
                )
                .await
                .unwrap()
            };
            format!("joined game: `{}`", game_token)
        }
    }
}
