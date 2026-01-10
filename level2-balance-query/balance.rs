use ethers::{
    providers::{Provider, Http, Middleware},
    types::Address,
    utils::format_units,
};
use std::str::FromStr;

const ARBITRUM_TESTNET_RPC: &str = "https://sepolia-rollup.arbitrum.io/rpc";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置要查询的地址
    let address_str = "0x5D8f25f74c09DeE91128CB4329e4ce9f14b147eF";
    
    println!("=== Arbitrum 测试网 ETH 余额查询 ===");
    println!("查询地址: {}", address_str);
    println!("网络: Arbitrum Sepolia Testnet");
    println!("RPC: {}", ARBITRUM_TESTNET_RPC);
    
    // 解析地址
    let address = Address::from_str(address_str)?;
    
    // 创建 Provider 连接
    let provider = Provider::<Http>::try_from(ARBITRUM_TESTNET_RPC)?;
    
    // 查询余额
    let balance_wei = provider.get_balance(address, None).await?;
    
    // 将 wei 转换为 ETH
    let balance_eth = format_units(balance_wei, "ether")?;
    
    println!("\n查询结果:");
    println!("地址: {}", address);
    println!("余额 (Wei): {}", balance_wei);
    println!("余额 (ETH): {} ETH", balance_eth);
    
    Ok(())
}
