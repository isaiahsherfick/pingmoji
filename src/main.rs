use clap::Parser;
use ping_rs::{PingOptions, send_ping};
use std::time::Duration;

#[derive(Debug)]
pub enum PingmojiError {
    NotEnoughBytes(String),
    InvalidOps(String),
    NotEnoughOps(String),
    TooManyOps(String),
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    emojis: String,

    #[arg(short, long, default_value_t = String::from(""))]
    operations: String,
}

fn get_ipv4_address_from_emoji(emoji: &str) -> Result<String, PingmojiError> {
    let bytes = emoji.bytes().collect::<Vec<u8>>();
    if bytes.len() < 4 {
        return Err(PingmojiError::NotEnoughBytes(emoji.to_string()));
    }
    return Ok(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]).to_string());
}

fn validate_ops(ops: String, num_emojis: usize) -> Result<String, PingmojiError> {
    let required_num_ops = num_emojis - 1;
    let num_ops = ops.chars().count();
    if num_ops > required_num_ops {
        println!(
            "Error: Number of bitwise operations must be equal to the number of emojis minus one."
        );
        println!(
            "Expected ops string of length {required_num_ops}, found ops string of length {num_ops}."
        );
        return Err(PingmojiError::TooManyOps(ops));
    } else if num_ops < required_num_ops {
        println!(
            "Error: Number of bitwise operations must be equal to the number of emojis minus one."
        );
        return Err(PingmojiError::NotEnoughOps(ops));
    }
    for c in ops.chars() {
        match c {
            'a' | 'A' | 'x' | 'X' | 'o' | 'O' => {}
            _ => {
                println!("Error: {ops} is an invalid bitwise operations argument.");
                println!("Valid operation characters:");
                println!("A: bitwise AND");
                println!("O: bitwise OR");
                println!("X: bitwise XOR");
                return Err(PingmojiError::InvalidOps(ops));
            }
        }
    }
    Ok(ops)
}

fn main() {
    let cli_args = Args::parse();
    let num_emojis = cli_args.emojis.chars().count();
    let ops = validate_ops(cli_args.operations, num_emojis).unwrap();
    let first_emoji = cli_args.emojis.chars().nth(0).unwrap();
    let mut target_addr = get_ipv4_address_from_emoji(&first_emoji.to_string()).unwrap();
    for i in 1..num_emojis {
        let emoji = cli_args.emojis.chars().nth(i).unwrap();
        let op = ops.chars().nth(i - 1).unwrap();
        let addr = get_ipv4_address_from_emoji(&emoji.to_string()).unwrap();
        let long_op = match op {
            'a' | 'A' => "bitwise AND",
            'x' | 'X' => "bitwise XOR",
            'o' | 'O' => "bitwise OR",
            _ => panic!("AAAAAAAAAAAAHHHHHH"),
        };
        println!("{} as ipv4 address: {}", emoji, addr);
        println!("\t{}", long_op);
        target_addr = perform_bitwise_op(&addr, &target_addr, op);
        println!("\t{}", target_addr);
    }
    let data = [1, 2, 3, 4]; // ping data
    let timeout = Duration::from_secs(5);
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    println!("Pinging {target_addr} with 4 bytes of data...");
    let result = ping_rs::send_ping(
        &target_addr.parse().unwrap(),
        timeout,
        &data,
        Some(&options),
    );
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

fn perform_bitwise_op(addr1: &str, addr2: &str, op: char) -> String {
    let split1 = addr1.split(".").collect::<Vec<&str>>();
    let split2 = addr2.split(".").collect::<Vec<&str>>();
    let mut result_arr: Vec<String> = vec![];
    for i in 0..split1.len() {
        let octet1 = split1[i].to_string().parse::<u8>().unwrap();
        let octet2 = split2[i].to_string().parse::<u8>().unwrap();
        match op {
            'a' | 'A' => {
                let result = octet1 & octet2;
                result_arr.push(result.to_string());
            }
            'x' | 'X' => {
                let result = octet1 ^ octet2;
                result_arr.push(result.to_string());
            }
            'o' | 'O' => {
                let result = octet1 | octet2;
                result_arr.push(result.to_string());
            }
            _ => panic!("AAAAAAAAAA"),
        }
    }
    let mut final_addr = result_arr[0].clone();
    for i in 1..result_arr.len() {
        final_addr += &format!(".{}", result_arr[i]);
    }
    final_addr
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_perform_bitwise_op() {
        let addr1 = "255.255.255.255";
        let addr2 = "123.123.0.45";
        let expected_and = "123.123.0.45";
        let expected_or = "255.255.255.255";
        let expected_xor = "132.132.255.210";
        assert_eq!(perform_bitwise_op(addr1, addr2, 'a'), expected_and);
        assert_eq!(perform_bitwise_op(addr1, addr2, 'A'), expected_and);
        assert_eq!(perform_bitwise_op(addr1, addr2, 'o'), expected_or);
        assert_eq!(perform_bitwise_op(addr1, addr2, 'O'), expected_or);
        assert_eq!(perform_bitwise_op(addr1, addr2, 'x'), expected_xor);
        assert_eq!(perform_bitwise_op(addr1, addr2, 'X'), expected_xor);
    }
}
