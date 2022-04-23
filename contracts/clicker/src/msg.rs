use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std:: Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub speed: i32,
    pub name: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Increment {},
    // Reset { speed: i32, name: i32 }

    UpsertScore { score: u16 } // add or update score
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetSpeed returns the current speed as a json-encoded number
    GetSpeed {},
    GetName {},
    GetScores {}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpeedResponse {
    pub speed: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NameResponse {
    pub name: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreResponse {
    pub scores: Vec<(Addr, u16)>,
}