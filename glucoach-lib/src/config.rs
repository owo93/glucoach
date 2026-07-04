use std::env::var;
use std::net::{IpAddr, Ipv4Addr};

use anyhow::Context as _;

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub supabase_url: String,
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        let host = var("HOST")
            .unwrap_or_else(|_| Ipv4Addr::LOCALHOST.to_string())
            .parse()
            .expect("HOST must be a valid IP address");

        let port = var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        let supabase_url = var("SUPABASE_URL").context("SUPABASE_URL must be set")?;

        Ok(Self {
            host,
            port,
            supabase_url,
        })
    }
}
