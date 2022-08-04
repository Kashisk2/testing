use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::Serialize;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, PartialEq, Serialize)]
pub enum AddressChain {
    Bitcoin(&'static str),
    Ethereum,
    Solana,
    Invalid,
    Unknown,
}

pub fn detect_address(addr: &str) -> AddressChain {
    if addr.len() < 34 {
        return AddressChain::Invalid;
    }

    if &addr[..3] == "bc1" {
        return AddressChain::Bitcoin("Bech32");
    } else if &addr[..2] == "0x" {
        // try ethereum
        if validate_eth_address(addr) {
            return AddressChain::Ethereum;
        }
        return AddressChain::Invalid;
    } else {
        // try solana
        if let Ok(decoded) = bs58::decode(addr).into_vec() {
            if decoded.len() == 32 && Pubkey::new(&decoded).is_on_curve() {
                return AddressChain::Solana;
            }
        }

        // try bitcoin here as we are only checking the first character
        if &addr[..1] == "1" {
            return AddressChain::Bitcoin("P2PKH");
        } else if &addr[..1] == "3" {
            return AddressChain::Bitcoin("P2SH");
        }
    }
    AddressChain::Unknown
}

fn validate_eth_address(address: &str) -> bool {
    let check = eth_checksum_encode(&address);
    if check == address {
        return true;
    }
    eth_checksum_encode(&check) == check
}

fn eth_checksum_encode(address: &str) -> String {
    let input = String::from(address.to_ascii_lowercase().trim_start_matches("0x"));
    let mut hasher = Sha3::keccak256();
    hasher.input_str(&input);
    let hex = hasher.result_str();
    let mut ret = String::with_capacity(42);
    ret.push_str("0x");
    for i in 0..40 {
        if u32::from_str_radix(&hex[i..i + 1], 16).unwrap() > 7 {
            ret.push_str(&address[i + 2..i + 3].to_ascii_uppercase());
        } else {
            ret.push_str(&address[i + 2..i + 3]);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLANA_TEST_ADDR: &str = "GfJ8bxuZeugbYeKiKe9Ga2eSFyZA2KksBbmvCN3zYSow";
    const BITCOIN_P2PKH_TEST_ADDR: &str = "1BoatSLRHtKNngkdXEeobR76b53LETtpyT";
    const BITCOIN_P2SH_TEST_ADDR: &str = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
    const BITCOIN_BECH32_TEST_ADDR: &str = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";
    const ETHEREUM_TEST_ADDR: &str = "0xb794f5ea0ba39494ce839613fffba74279579268";

    #[test]
    fn test_detect_address() {
        assert_eq!(
            detect_address(SOLANA_TEST_ADDR),
            AddressChain::Solana,
            "solana"
        );
        assert_eq!(
            detect_address(BITCOIN_P2PKH_TEST_ADDR),
            AddressChain::Bitcoin("P2PKH"),
            "bitcoin(p2pkh)"
        );
        assert_eq!(
            detect_address(BITCOIN_P2SH_TEST_ADDR),
            AddressChain::Bitcoin("P2SH")
        );
        assert_eq!(
            detect_address(BITCOIN_BECH32_TEST_ADDR),
            AddressChain::Bitcoin("Bech32")
        );
        assert_eq!(
            detect_address(ETHEREUM_TEST_ADDR),
            AddressChain::Ethereum,
            "ethereum"
        );
    }

    #[test]
    fn test_detect_address_invalid() {
        assert_eq!(
            detect_address("INVALID_ADDR_LEN"),
            AddressChain::Invalid,
            "should be invalid"
        );
    }
}
