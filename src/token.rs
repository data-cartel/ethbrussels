#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    USDC(USDC),
    USDT(USDT),
    NEAR(NEAR),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct USDC {
    symbol: &'static str,
    address: &'static str,
    decimals: u8,
}
// TODO this is fuckin ugly, surely there is a better way to represent tokens haha
impl USDC {
    pub fn new() -> Self {
        USDC {
            symbol: "USDC",
            address: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
            //TODO verify
            decimals: 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct USDT {
    symbol: &'static str,
    address: &'static str,
    decimals: u8,
}

impl USDT {
    pub fn new() -> Self {
        USDT {
            symbol: "USDT",
            address: "usdt.tether-token.near",
            //TODO verify
            decimals: 18,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NEAR {
    symbol: &'static str,
    address: &'static str,
    decimals: u8,
}

impl NEAR {
    pub fn new() -> Self {
        NEAR {
            symbol: "NEAR",
            address: "wrap.near",
            //TODO verify
            decimals: 24,
        }
    }
}
