use near_sdk::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    symbol: String,
    address: String,
    decimals: u8,
}

impl Token {
    pub fn near() -> Self {
        Token {
            symbol: "NEAR".to_string(),
            address: "wrap.near".to_string(),
            decimals: 24,
        }
    }

    pub fn usdt() -> Self {
        Token {
            symbol: "USDT".to_string(),
            address: "usdt.tether-token.near".to_string(),
            decimals: 6,
        }
    }

    pub fn usdc() -> Self {
        Token {
            symbol: "USDC".to_string(),
            address: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".to_string(),
            decimals: 6,
        }
    }
}
