//! Predict-server REST client.
//! https://predict-server.testnet.mystenlabs.com — exposes /oracles, /managers.

use anyhow::Result;
use serde::Deserialize;

use crate::config;

// v2 sources the oracle/expiry-market list on-chain via `predict-cli list`,
// so the predict-server `/oracles` response shape is no longer wired into
// the CLI. The PredictManager lookup remains a server call because the
// `PredictManagerCreated` event index is the only easy way to recover a
// user's manager from their address.

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ServerManager {
    pub manager_id: String,
    pub owner: String,
    pub digest: String,
    pub checkpoint: u64,
    pub checkpoint_timestamp_ms: u64,
}

pub async fn list_managers() -> Result<Vec<ServerManager>> {
    let url = format!("{}/managers", config::predict_server());
    let res = reqwest::get(&url)
        .await?
        .json::<Vec<ServerManager>>()
        .await?;
    Ok(res)
}

pub async fn find_manager_for(owner: &str) -> Result<Option<ServerManager>> {
    let all = list_managers().await?;
    Ok(all.into_iter().rev().find(|m| m.owner == owner))
}
