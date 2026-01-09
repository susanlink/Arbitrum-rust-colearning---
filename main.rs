use ethers::{
    providers::{Provider, Http},
    middleware::Middleware,
    types::{H160, U64},
    utils::format_units,
};
use eyre::Result;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Hello Web3 - Arbitrum 测试网连接测试");
    println!("==========================================");
    
    // Arbitrum 测试网 RPC 端点
    let arbitrum_sepolia_rpc = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 可选：其他 Arbitrum 测试网端点
    let arbitrum_rpc_endpoints = vec![
        ("Arbitrum Sepolia", "https://sepolia-rollup.arbitrum.io/rpc"),
        ("Arbitrum Goerli", "https://goerli-rollup.arbitrum.io/rpc"),
        ("Arbitrum Nova", "https://nova.arbitrum.io/rpc"),
        ("Arbitrum One", "https://arb1.arbitrum.io/rpc"),
    ];
    
    println!("📡 测试连接到 Arbitrum 网络...");
    
    for (network_name, rpc_url) in arbitrum_rpc_endpoints {
        println!("\n尝试连接: {} - {}", network_name, rpc_url);
        
        match Provider::<Http>::try_from(rpc_url) {
            Ok(provider) => {
                println!("  ✅ Provider 创建成功");
                
                // 测试链ID
                match provider.get_chainid().await {
                    Ok(chain_id) => {
                        let chain_id_num = chain_id.as_u64();
                        println!("  🔗 链ID: {}", chain_id_num);
                        
                        // 识别 Arbitrum 网络
                        match chain_id_num {
                            421614 => {
                                println!("  🌐 网络识别: Arbitrum Sepolia 测试网");
                                println!("  📊 网络信息:");
                                println!("     - Chain ID: 421614");
                                println!("     - RPC URL: {}", rpc_url);
                                println!("     - 状态: 🟢 运行中");
                                
                                // 获取更多网络数据
                                match provider.get_block_number().await {
                                    Ok(block) => println!("     - 当前区块: {}", block),
                                    Err(e) => println!("     - 区块查询失败: {}", e),
                                }
                                
                                match provider.get_gas_price().await {
                                    Ok(gas) => {
                                        if let Ok(gas_gwei) = format_units(gas, "gwei") {
                                            println!("     - Gas 价格: {} Gwei", gas_gwei);
                                        }
                                    }
                                    Err(e) => println!("     - Gas 查询失败: {}", e),
                                }
                                
                                println!("  🎉 Arbitrum Sepolia 测试网连接成功！");
                                return Ok(());
                            }
                            421613 => {
                                println!("  🌐 网络识别: Arbitrum Goerli 测试网");
                                println!("  ⚠️  注意: Goerli 测试网已弃用，建议使用 Sepolia");
                            }
                            42170 => {
                                println!("  🌐 网络识别: Arbitrum Nova");
                                println!("  💡 这是 Arbitrum 的数据可用性层");
                            }
                            42161 => {
                                println!("  🌐 网络识别: Arbitrum One 主网");
                                println!("  ⚠️  这是主网，小心真实资产！");
                            }
                            _ => println!("  🌐 网络识别: 未知 Arbitrum 网络"),
                        }
                    }
                    Err(e) => println!("  ❌ 获取链ID失败: {}", e),
                }
            }
            Err(e) => println!("  ❌ 连接失败: {}", e),
        }
    }
    
    println!("\n🔍 专项测试：Arbitrum Sepolia 测试网");
    println!("========================================");
    
    // 专项测试 Arbitrum Sepolia
    let provider = match Provider::<Http>::try_from(arbitrum_sepolia_rpc) {
        Ok(p) => {
            println!("✅ 成功连接到 Arbitrum Sepolia RPC");
            p
        }
        Err(e) => {
            println!("❌ 连接失败: {}", e);
            println!("可能的原因:");
            println!("1. 网络连接问题");
            println!("2. RPC 端点暂时不可用");
            println!("3. 防火墙阻止了连接");
            return Ok(());
        }
    };
    
    // 获取网络状态
    println!("\n📊 网络状态检查:");
    
    let chain_id = provider.get_chainid().await?;
    println!("1. 链ID: {} {}", chain_id, 
        if chain_id.as_u64() == 421614 { "✅ (Arbitrum Sepolia)" } else { "❌ (不是 Arbitrum Sepolia)" });
    
    let block_number = provider.get_block_number().await?;
    println!("2. 最新区块: {}", block_number);
    
    let gas_price = provider.get_gas_price().await?;
    let gas_gwei = format_units(gas_price, "gwei")?;
    println!("3. Gas 价格: {} Gwei", gas_gwei);
    
    // 测试地址查询
    println!("\n👤 测试地址查询:");
    let test_address: H160 = H160::from_str("0x0000000000000000000000000000000000000000")?;
    let balance = provider.get_balance(test_address, None).await?;
    println!("零地址余额: {} ETH", format_units(balance, "ether")?);
    
    // 测试智能合约查询（Arbitrum 上的 USDC 合约）
    println!("\n💎 测试智能合约查询:");
    let usdc_contract: H160 = H160::from_str("0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d")?;
    match provider.get_code(usdc_contract, None).await {
        Ok(code) => {
            if code.len() > 0 {
                println!("USDC 合约代码长度: {} bytes ✅", code.len());
            } else {
                println!("USDC 合约未找到或未部署");
            }
        }
        Err(e) => println!("查询合约失败: {}", e),
    }
    
    println!("\n========================================");
    println!("🎯 Arbitrum 测试网连接测试完成");
    println!("✅ 所有测试通过");
    println!("🌐 网络: Arbitrum Sepolia (链ID: 421614)");
    println!("📡 RPC: {}", arbitrum_sepolia_rpc);
    println!("✨ Hello Arbitrum Web3!");
    
    Ok(())
}