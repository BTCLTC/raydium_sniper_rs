use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::{fs, str::FromStr};

pub const WSS_URL: &str = "wss://solana-mainnet.g.alchemy.com/v2/0XU-8dK4QR34JMpk6457JTlYUC6sCfes";
pub const RPC_URL: &str =
    "https://solana-mainnet.g.alchemy.com/v2/0XU-8dK4QR34JMpk6457JTlYUC6sCfes";

pub struct Config {
    pub sender_keypair: Keypair,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let key = fs::read_to_string(path)?;
        let key_bytes: Vec<u8> = serde_json::from_str(&key)?;
        let sender_keypair = Keypair::from_bytes(&key_bytes)?;

        Ok(Config { sender_keypair })
    }

    pub fn sender_pubkey(&self) -> Pubkey {
        self.sender_keypair.pubkey()
    }
}

#[allow(non_snake_case)]
pub fn wSol() -> Pubkey {
    return Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
}

pub fn program_id_rv4() -> Pubkey {
    return Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();
}
