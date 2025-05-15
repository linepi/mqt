use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// 策略类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StrategyType {
    Momentum,     // 动量策略
    MeanReversion, // 均值回归策略
    PairTrading,  // 配对交易
    FactorModel,  // 因子模型
    Custom,       // 自定义策略
}

// 策略参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParams {
    // 通用参数
    pub name: String,
    pub description: String,
    pub strategy_type: StrategyType,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // 风险控制参数
    pub max_position_size: f64,     // 单个持仓最大比例
    pub max_drawdown: f64,          // 最大回撤限制
    pub stop_loss: f64,             // 止损比例
    pub take_profit: f64,           // 止盈比例
    
    // 策略特定参数
    pub params: HashMap<String, serde_json::Value>,
}

impl StrategyParams {
    pub fn new(name: String, strategy_type: StrategyType) -> Self {
        Self {
            name,
            description: String::new(),
            strategy_type,
            enabled: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            max_position_size: 0.1,
            max_drawdown: 0.2,
            stop_loss: 0.05,
            take_profit: 0.1,
            params: HashMap::new(),
        }
    }
    
    // 获取参数值
    pub fn get_param<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.params.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    // 设置参数值
    pub fn set_param<T>(&mut self, key: &str, value: T) -> Result<(), serde_json::Error>
    where
        T: serde::Serialize,
    {
        let json_value = serde_json::to_value(value)?;
        self.params.insert(key.to_string(), json_value);
        self.updated_at = Utc::now();
        Ok(())
    }
}

// 回测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    // 策略信息
    pub strategy_name: String,
    pub strategy_type: StrategyType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    
    // 回测参数
    pub initial_capital: f64,
    pub final_capital: f64,
    
    // 绩效指标
    pub total_return: f64,
    pub annualized_return: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    
    // 交易统计
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub avg_profit: f64,
    pub avg_loss: f64,
    
    // 详细数据
    pub equity_curve: Vec<(DateTime<Utc>, f64)>,
    pub trades: Vec<BacktestTrade>,
}

// 回测交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestTrade {
    pub code: String,
    pub entry_date: DateTime<Utc>,
    pub entry_price: f64,
    pub entry_amount: f64,
    pub exit_date: Option<DateTime<Utc>>,
    pub exit_price: Option<f64>,
    pub exit_amount: Option<f64>,
    pub profit_loss: Option<f64>,
    pub profit_loss_percent: Option<f64>,
    pub exit_reason: Option<String>,
}

// 策略信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub code: String,
    pub timestamp: DateTime<Utc>,
    pub action: SignalAction,
    pub price: Option<f64>,
    pub amount: Option<f64>,
    pub reason: String,
    pub strength: f64, // 信号强度, 0-1
}

// 信号动作
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

// 市场数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub timestamp: DateTime<Utc>,
    pub stocks: HashMap<String, StockSnapshot>,
}

// 股票快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSnapshot {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub turnover: f64,
    pub change_percent: f64,
} 