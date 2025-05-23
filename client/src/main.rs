use std::io::{self, Write};
use reqwest::Client;
use serde_json::Value;
use log::info;
use server::stockdata::FetchRequest;
use common::constants::BASE_URL;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置环境变量
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("LANG", "zh_CN.UTF-8");
    
    // 初始化日志系统
    env_logger::Builder::new()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] ({}:{}) - {}",
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
    
    info!("启动交易系统客户端...");
    
    // 创建HTTP客户端
    let client = Client::new();
    let base_url = BASE_URL;
    
    println!("欢迎使用量化交易系统客户端");
    println!("输入 'help' 查看可用命令, 输入 'exit' 退出");
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("退出客户端");
                break;
            },
            "help" => {
                print_help();
            },
            "status" => {
                check_server_status(&client, base_url).await?;
            },
            cmd if cmd.starts_with("stockdata ") => {
                handle_stockdata_command(&client, base_url, &cmd[10..]).await?;
            },
            cmd if cmd.starts_with("position ") => {
                handle_position_command(&client, base_url, &cmd[9..]).await?;
            },
            cmd if cmd.starts_with("strategy ") => {
                handle_strategy_command(&client, base_url, &cmd[9..]).await?;
            },
            _ => {
                println!("未知命令, 输入 'help' 查看可用命令");
            }
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("可用命令：");
    println!("  help                    - 显示帮助信息");
    println!("  exit, quit              - 退出客户端");
    println!("  status                  - 检查服务器状态");
    println!("  stockdata price <code>  - 获取股票价格");
    println!("  stockdata init          - 初始化股票数据抓取器");
    println!("  stockdata fetch         - 抓取股票数据");
    println!("  stockdata close         - 关闭股票数据抓取器");
    println!("  stockdata status        - 查看股票数据抓取器状态");
    println!("  position list           - 列出当前持仓");
    println!("  position query_portfolio <name> - 查询投资组合信息");
    println!("  position add_portfolio <name> <cash_balance> - 添加投资组合");
    println!("  position remove_portfolio <name> - 删除投资组合");
    println!("  position add <portfolio> <code> <amount> - 添加持仓");
    println!("  position remove <portfolio> <code> <amount> - 减少持仓");
    println!("  strategy list           - 列出可用策略");
    println!("  strategy run <name>     - 运行策略");
    println!("  strategy backtest <name> - 回测策略");
}

async fn check_server_status(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(format!("{}/status", base_url)).send().await?;
    
    if response.status().is_success() {
        let status: Value = response.json().await?;
        println!("服务器状态: {}", status);
    } else {
        println!("无法连接到服务器, 请确认服务器是否运行");
    }
    
    Ok(())
}

async fn handle_stockdata_command(client: &Client, base_url: &str, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        "init" => {
            let response = client.post(format!("{}/stockdata/init", base_url)).send().await?;
            if response.status().is_success() {
                println!("股票数据抓取器初始化成功");
            } else {
                println!("股票数据抓取器初始化失败: {}", response.text().await?);
            }
        },
        "fetch" => {
            let response = client.post(format!("{}/stockdata/fetch", base_url))
                .json(&FetchRequest { save_to_file: Some(true) })
                .send().await?;
            if response.status().is_success() {
                println!("股票数据抓取成功");
            } else {
                println!("股票数据抓取失败: {}", response.text().await?);
            }
        },
        "close" => {
            let response = client.post(format!("{}/stockdata/close", base_url)).send().await?;
            if response.status().is_success() {
                println!("股票数据抓取器关闭成功");
            } else {
                println!("股票数据抓取器关闭失败: {}", response.text().await?);
            }
        },
        "status" => {
            let response = client.get(format!("{}/stockdata/status", base_url)).send().await?;
            if response.status().is_success() {
                let status: Value = response.json().await?;
                println!("股票数据抓取器状态: {}", status);
            } else {
                println!("获取状态失败: {}", response.text().await?);
            }
        },
        "price" => {
            let response = client.get(format!("{}/stockdata/price", base_url)).send().await?;
            if response.status().is_success() {
                let price: Value = response.json().await?;
                println!("股票价格: {}", price);
            }
        },
        _ => {
            println!("未知的股票数据命令");
        }
    }
    
    Ok(())
}

async fn handle_position_command(client: &Client, base_url: &str, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cmd == "list" {
        let response = client.get(format!("{}/position/list", base_url)).send().await?;
        if response.status().is_success() {
            let positions: Value = response.json().await?;
            println!("当前持仓: {}", positions);
        } else {
            println!("获取持仓失败: {}", response.text().await?);
        }
    } else if cmd.starts_with("query_portfolio ") {
        let parts: Vec<&str> = cmd[16..].split_whitespace().collect();
        if parts.len() >= 1 {
            let name = parts[0];
            let response = client.post(format!("{}/position/query_portfolio", base_url))
                .json(&serde_json::json!({
                    "name": name
                }))
                .send().await?;
            if response.status().is_success() {
                let portfolio: server::position::QueryPortfolioResponse = response.json().await?;
                println!("投资组合信息: {}", serde_json::to_string(&portfolio).unwrap());
            } else {
                println!("查询投资组合失败: {}", response.text().await?);
            }
        } else {
            println!("用法: position query_portfolio <name>");
        }
    } else if cmd.starts_with("add_portfolio ") {
        let parts: Vec<&str> = cmd[14..].split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0];
            let cash_balance: f64 = parts[1].parse()?;
            
            let response = client.post(format!("{}/position/add_portfolio", base_url))
                .json(&serde_json::json!({
                    "name": name,
                    "cash_balance": cash_balance
                }))
                .send().await?;
                
            if response.status().is_success() {
                println!("添加投资组合成功");
            } else {
                println!("添加投资组合失败: {}", response.text().await?);
            }
        } else {
            println!("用法: position add_portfolio <name> <cash_balance>");
        }
    } else if cmd.starts_with("remove_portfolio ") {
        let parts: Vec<&str> = cmd[17..].split_whitespace().collect();
        if parts.len() >= 1 {
            let name = parts[0];
            
            let response = client.post(format!("{}/position/remove_portfolio", base_url))
                .json(&serde_json::json!({
                    "name": name,
                }))
                .send().await?;
                
            if response.status().is_success() {
                println!("删除投资组合成功");
            } else {
                println!("删除投资组合失败: {}", response.text().await?);
            }
        } else {
            println!("用法: position remove_portfolio <name>");
        }
    } else if cmd.starts_with("add ") {
        let parts: Vec<&str> = cmd[4..].split_whitespace().collect();
        if parts.len() >= 3 {
            let portfolio = parts[0];
            let code = parts[1];
            let amount: f64 = parts[2].parse()?;
            
            let response = client.post(format!("{}/position/add", base_url))
                .json(&serde_json::json!({
                    "portfolio": portfolio,
                    "code": code,
                    "amount": amount
                }))
                .send().await?;
                
            if response.status().is_success() {
                println!("添加持仓成功");
            } else {
                println!("添加持仓失败: {}", response.text().await?);
            }
        } else {
            println!("用法: position add <portfolio> <code> <amount>");
        }
    } else if cmd.starts_with("remove ") {
        let parts: Vec<&str> = cmd[7..].split_whitespace().collect();
        if parts.len() >= 3 {
            let portfolio = parts[0];
            let code = parts[1];
            let amount: f64 = parts[2].parse()?;
            
            let response = client.post(format!("{}/position/remove", base_url))
                .json(&serde_json::json!({
                    "portfolio": portfolio,
                    "code": code,
                    "amount": amount
                }))
                .send().await?;
                
            if response.status().is_success() {
                println!("减少持仓成功");
            } else {
                println!("减少持仓失败: {}", response.text().await?);
            }
        } else {
            println!("用法: position remove <portfolio> <code> <amount>");
        }
    } else {
        println!("未知的持仓命令");
    }
    
    Ok(())
}

async fn handle_strategy_command(client: &Client, base_url: &str, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cmd == "list" {
        let response = client.get(format!("{}/strategy/list", base_url)).send().await?;
        if response.status().is_success() {
            let strategies: Value = response.json().await?;
            println!("可用策略: {}", strategies);
        } else {
            println!("获取策略列表失败: {}", response.text().await?);
        }
    } else if cmd.starts_with("run ") {
        let strategy_name = &cmd[4..];
        
        let response = client.post(format!("{}/strategy/run", base_url))
            .json(&serde_json::json!({
                "name": strategy_name
            }))
            .send().await?;
            
        if response.status().is_success() {
            println!("策略运行成功");
        } else {
            println!("策略运行失败: {}", response.text().await?);
        }
    } else if cmd.starts_with("backtest ") {
        let strategy_name = &cmd[9..];
        
        let response = client.post(format!("{}/strategy/backtest", base_url))
            .json(&serde_json::json!({
                "name": strategy_name
            }))
            .send().await?;
            
        if response.status().is_success() {
            let result: Value = response.json().await?;
            println!("回测结果: {}", result);
        } else {
            println!("策略回测失败: {}", response.text().await?);
        }
    } else {
        println!("未知的策略命令");
    }
    
    Ok(())
} 