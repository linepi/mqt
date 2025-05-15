use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// 持仓记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    // 股票代码
    pub code: String,
    // 股票名称
    pub name: String,
    // 持仓数量
    pub amount: f64,
    // 持仓成本
    pub cost: f64,
    // 当前价格
    pub current_price: Option<f64>,
    // 最后更新时间
    pub last_update: DateTime<Utc>,
    // 交易记录
    pub transactions: Vec<Transaction>,
}

impl Position {
    pub fn new(code: String, name: String, amount: f64, cost: f64) -> Self {
        Self {
            code,
            name,
            amount,
            cost,
            current_price: None,
            last_update: Utc::now(),
            transactions: Vec::new(),
        }
    }
    
    // 计算当前市值
    pub fn market_value(&self) -> Option<f64> {
        self.current_price.map(|price| price * self.amount)
    }
    
    // 计算持仓成本
    pub fn total_cost(&self) -> f64 {
        self.cost * self.amount
    }
    
    // 计算盈亏
    pub fn profit_loss(&self) -> Option<f64> {
        self.market_value().map(|value| value - self.total_cost())
    }
    
    // 计算盈亏比例
    pub fn profit_loss_percent(&self) -> Option<f64> {
        if self.total_cost() == 0.0 {
            return None;
        }
        
        self.profit_loss().map(|pl| pl / self.total_cost() * 100.0)
    }
    
    // 添加交易记录
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_position_from_transactions();
    }
    
    // 根据交易记录更新持仓
    fn update_position_from_transactions(&mut self) {
        let mut total_amount = 0.0;
        let mut total_cost = 0.0;
        
        for transaction in &self.transactions {
            match transaction.transaction_type {
                TransactionType::Buy => {
                    total_amount += transaction.amount;
                    total_cost += transaction.amount * transaction.price;
                },
                TransactionType::Sell => {
                    total_amount -= transaction.amount;
                    // 卖出时不减少总成本, 而是在计算平均成本时调整分母
                }
            }
        }
        
        self.amount = total_amount;
        
        if total_amount > 0.0 {
            self.cost = total_cost / total_amount;
        } else {
            self.cost = 0.0;
        }
        
        self.last_update = Utc::now();
    }
}

// 交易类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionType {
    Buy,
    Sell,
}

// 交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    // 交易ID
    pub id: String,
    // 股票代码
    pub code: String,
    // 交易类型
    pub transaction_type: TransactionType,
    // 交易数量
    pub amount: f64,
    // 交易价格
    pub price: f64,
    // 交易时间
    pub timestamp: DateTime<Utc>,
    // 交易费用
    pub fee: Option<f64>,
    // 交易备注
    pub note: Option<String>,
}

impl Transaction {
    pub fn new(code: String, transaction_type: TransactionType, amount: f64, price: f64) -> Self {
        Self {
            id: format!("{}", uuid::Uuid::new_v4()),
            code,
            transaction_type,
            amount,
            price,
            timestamp: Utc::now(),
            fee: None,
            note: None,
        }
    }
    
    // 计算交易金额
    pub fn total_value(&self) -> f64 {
        self.amount * self.price
    }
}

// 投资组合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    // 投资组合名称
    pub name: String,
    // 持仓列表
    pub positions: HashMap<String, Position>,
    // 现金余额
    pub cash_balance: f64,
    // 创建时间
    pub created_at: DateTime<Utc>,
    // 最后更新时间
    pub last_update: DateTime<Utc>,
}

impl Portfolio {
    pub fn new(name: String, cash_balance: f64) -> Self {
        Self {
            name,
            positions: HashMap::new(),
            cash_balance,
            created_at: Utc::now(),
            last_update: Utc::now(),
        }
    }
    
    // 获取总市值
    pub fn total_market_value(&self) -> f64 {
        let positions_value: f64 = self.positions.values()
            .filter_map(|p| p.market_value())
            .sum();
        
        positions_value + self.cash_balance
    }
    
    // 获取总成本
    pub fn total_cost(&self) -> f64 {
        let positions_cost: f64 = self.positions.values()
            .map(|p| p.total_cost())
            .sum();
        
        positions_cost + self.cash_balance
    }
    
    // 计算总盈亏
    pub fn total_profit_loss(&self) -> f64 {
        self.total_market_value() - self.total_cost()
    }
    
    // 计算总盈亏比例
    pub fn total_profit_loss_percent(&self) -> Option<f64> {
        let total_cost = self.total_cost();
        if total_cost == 0.0 {
            return None;
        }
        
        Some(self.total_profit_loss() / total_cost * 100.0)
    }
    
    // 添加持仓
    pub fn add_position(&mut self, position: Position) {
        self.positions.insert(position.code.clone(), position);
        self.last_update = Utc::now();
    }
    
    // 移除持仓
    pub fn remove_position(&mut self, code: &str) -> Option<Position> {
        let position = self.positions.remove(code);
        if position.is_some() {
            self.last_update = Utc::now();
        }
        position
    }
    
    // 添加交易
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        let code = transaction.code.clone();
        
        match transaction.transaction_type {
            TransactionType::Buy => {
                // 检查现金余额是否足够
                let total_cost = transaction.total_value();
                if total_cost > self.cash_balance {
                    return Err("现金余额不足".to_string());
                }
                
                // 扣除现金
                self.cash_balance -= total_cost;
                
                // 更新持仓
                if let Some(position) = self.positions.get_mut(&code) {
                    position.add_transaction(transaction);
                } else {
                    // 创建新持仓
                    let mut position = Position::new(
                        code.clone(),
                        format!("Unknown Stock {}", code),
                        transaction.amount,
                        transaction.price
                    );
                    position.add_transaction(transaction);
                    self.positions.insert(code, position);
                }
            },
            TransactionType::Sell => {
                // 检查持仓是否足够
                if let Some(position) = self.positions.get(&code) {
                    if position.amount < transaction.amount {
                        return Err("持仓数量不足".to_string());
                    }
                } else {
                    return Err("没有该股票的持仓".to_string());
                }
                
                // 增加现金
                let total_value = transaction.total_value();
                self.cash_balance += total_value;
                
                // 更新持仓
                if let Some(position) = self.positions.get_mut(&code) {
                    position.add_transaction(transaction);
                    
                    // 如果持仓数量为0, 则移除持仓
                    if position.amount == 0.0 {
                        self.positions.remove(&code);
                    }
                }
            }
        }
        
        self.last_update = Utc::now();
        Ok(())
    }
} 