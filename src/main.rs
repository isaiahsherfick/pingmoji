use clap::Parser;
use ping_rs::{PingOptions, send_ping};
use std::time::Duration;

#[derive(Debug)]
pub enum PingmojiError {
    NotEnoughBytes(String),
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    emoji: String,
}

fn get_ipv4_address_from_emoji(emoji: &str, radix: usize) -> Result<String, PingmojiError> {
    let mut bytes = emoji.bytes().collect::<Vec<u8>>();
    if bytes.len() < 4 {
        return Err(PingmojiError::NotEnoughBytes(emoji.to_string()));
    }
    return Ok(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]).to_string());
}

fn error_out() {
    println!("You can only ping one emoji at a time");
    panic!("AHHHHHHH");
}

fn main() {
    let cli_args = Args::parse();
    let emoji = cli_args.emoji;
    match get_ipv4_address_from_emoji(&emoji, 4) {
        Ok(ip_addr) => {
            println!("ip_addr: {}", ip_addr);
            let addr = ip_addr.parse().unwrap();
            let data = [1, 2, 3, 4]; // ping data
            let timeout = Duration::from_secs(5);
            let options = ping_rs::PingOptions {
                ttl: 128,
                dont_fragment: true,
            };
            let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
            match result {
                Ok(reply) => println!(
                    "Reply from {}: bytes={} time={}ms TTL={}",
                    reply.address,
                    data.len(),
                    reply.rtt,
                    options.ttl
                ),
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
