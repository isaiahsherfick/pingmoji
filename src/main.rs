use ping_rs::{PingOptions,send_ping};
use std::time::Duration;

#[derive(Debug)]
pub enum PingmojiError {
    NotEnoughBytes(String)
}

fn get_ipv4_address_from_emoji(emoji: &str) -> Result<String,PingmojiError>{
    let mut bytes = emoji.bytes().collect::<Vec<u8>>();
    if bytes.len() < 4 {
        return Err(PingmojiError::NotEnoughBytes(emoji.to_string()));
    }
    return Ok(format!("{}.{}.{}.{}",bytes[0],bytes[1],bytes[2],bytes[3]).to_string());
}

//TODO: parse CLI args, ping address
fn main() {
    match get_ipv4_address_from_emoji("🍌") {
        Ok(ip_addr) => {
            println!("ip_addr: {}", ip_addr);
        }
        Err(e) => {
            println!("{:?}",e);
        }
    }
}
