use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Eq,PartialEq)]
pub struct GameInfo {
    pub players: Vec<u64>
}