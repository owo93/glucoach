use std::net::{IpAddr, Ipv4Addr};

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| Ipv4Addr::LOCALHOST.to_string())
            .parse()
            .expect("HOST must be a valid IP address");

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        Ok(Self { host, port })
    }
}
