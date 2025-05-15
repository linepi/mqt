use crate::models::{BacktestResult, BacktestTrade, StrategyType};
use log::info;

// 执行回测
pub fn run_backtest(
    strategy_name: &str,
    strategy_type: StrategyType,
    initial_capital: f64,
) -> BacktestResult {
    info!("执行回测: {}", strategy_name);
    
    // 这里只是创建一个模拟的回测结果
    // 实际应用中需要实现真正的回测逻辑
    
    BacktestResult {
        strategy_name: strategy_name.to_string(),
        strategy_type,
        start_date: chrono::Utc::now() - chrono::Duration::days(30),
        end_date: chrono::Utc::now(),
        initial_capital,
        final_capital: initial_capital * 1.1, // 假设收益10%
        total_return: 0.1,
        annualized_return: 0.12,
        sharpe_ratio: 1.5,
        max_drawdown: 0.05,
        win_rate: 0.6,
        profit_factor: 1.8,
        total_trades: 20,
        winning_trades: 12,
        losing_trades: 8,
        avg_profit: 1000.0,
        avg_loss: -500.0,
        equity_curve: vec![],
        trades: vec![
            BacktestTrade {
                code: "SH000001".to_string(),
                entry_date: chrono::Utc::now() - chrono::Duration::days(25),
                entry_price: 3000.0,
                entry_amount: 1.0,
                exit_date: Some(chrono::Utc::now() - chrono::Duration::days(20)),
                exit_price: Some(3100.0),
                exit_amount: Some(1.0),
                profit_loss: Some(100.0),
                profit_loss_percent: Some(3.33),
                exit_reason: Some("止盈".to_string()),
            },
            BacktestTrade {
                code: "SZ399001".to_string(),
                entry_date: chrono::Utc::now() - chrono::Duration::days(15),
                entry_price: 10000.0,
                entry_amount: 1.0,
                exit_date: Some(chrono::Utc::now() - chrono::Duration::days(10)),
                exit_price: Some(9800.0),
                exit_amount: Some(1.0),
                profit_loss: Some(-200.0),
                profit_loss_percent: Some(-2.0),
                exit_reason: Some("止损".to_string()),
            },
        ],
    }
} 