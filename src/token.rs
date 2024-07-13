pub enum Token {
    USDC(USDC),
    USDT(USDT),
    NEAR(NEAR)
}
struct USDC {
    symbol: String,
    address: String,
    decimals: u8,
}
struct USDT {
    symbol: String,
    address: String,
    decimals: u8,
}
struct NEAR {
    symbol: String,
    address: String,
    decimals: u8,
}