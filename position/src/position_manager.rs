use crate::models::{Portfolio, Position, Transaction, TransactionType};
use log::info;
use std::collections::HashMap;

pub struct PositionManager {
    pub portfolio: Portfolio,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            portfolio: Portfolio::new("默认组合".to_string(), 100000.0),
        }
    }
    
    // 添加持仓
    pub fn add_position(&mut self, code: String, _name: String, amount: f64, price: f64) -> Result<(), String> {
        info!("添加持仓: {} x {} @ {}", code, amount, price);
        
        // 创建交易记录
        let transaction = Transaction::new(
            code.clone(),
            TransactionType::Buy,
            amount,
            price,
        );
        
        // 添加交易
        self.portfolio.add_transaction(transaction)
    }
    
    // 减少持仓
    pub fn remove_position(&mut self, code: String, amount: f64, price: f64) -> Result<(), String> {
        info!("减少持仓: {} x {} @ {}", code, amount, price);
        
        // 检查持仓是否存在
        if !self.portfolio.positions.contains_key(&code) {
            return Err(format!("持仓 {} 不存在", code));
        }
        
        // 检查持仓数量是否足够
        let position = self.portfolio.positions.get(&code).unwrap();
        if position.amount < amount {
            return Err(format!("持仓数量不足, 当前持有 {}", position.amount));
        }
        
        // 创建交易记录
        let transaction = Transaction::new(
            code,
            TransactionType::Sell,
            amount,
            price,
        );
        
        // 添加交易
        self.portfolio.add_transaction(transaction)
    }
    
    // 更新持仓价格
    pub fn update_prices(&mut self, prices: HashMap<String, f64>) -> usize {
        let mut updated = 0;
        
        for (code, price) in prices {
            if let Some(position) = self.portfolio.positions.get_mut(&code) {
                position.current_price = Some(price);
                position.last_update = chrono::Utc::now();
                updated += 1;
            }
        }
        
        updated
    }
    
    // 获取所有持仓
    pub fn get_positions(&self) -> Vec<&Position> {
        self.portfolio.positions.values().collect()
    }
    
    // 获取特定持仓
    pub fn get_position(&self, code: &str) -> Option<&Position> {
        self.portfolio.positions.get(code)
    }
    
    // 获取投资组合
    pub fn get_portfolio(&self) -> &Portfolio {
        &self.portfolio
    }
} 