use consumet::models::StreamingServers;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, time::Duration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub addr: IpAddr,
    pub port: u16,
    #[serde(with = "humantime_serde")]
    pub shutdown_timeout: Option<Duration>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderInfo {
    pub intro: String,
    pub routes: Vec<String>,
    pub documentation: String,
}

#[derive(Debug, Deserialize)]
pub struct FlixhqSearch {
    pub query: String,
    pub page: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FlixhqInfo {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct FlixhqServer {
    pub episode_id: String,
    pub media_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FlixhqSource {
    pub episode_id: String,
    pub media_id: String,
    pub server: Option<StreamingServers>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub message: String,
    pub error: String,
}
