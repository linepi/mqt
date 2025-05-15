use crate::models::{Portfolio, Transaction, TransactionType};
use common::constants::BASE_URL;

pub struct PositionManager<'a> {
    pub portfolio: &'a mut Portfolio,
}

impl<'a> PositionManager<'a> {
    pub fn new(portfolio: &'a mut Portfolio) -> Self {
        Self { portfolio }
    }

    pub async fn new_transaction(&mut self, code: String, transaction_type: TransactionType, amount: f64) 
        -> Result<Transaction, Box<dyn std::error::Error>> {
        let url = format!("{}/stockdata/price?code={}", BASE_URL, code);
        let response = reqwest::get(url).await?;
        
        // 直接解析返回的单个对象
        let price: f64 = response.json().await?;

        let transaction = Transaction::new(
            code,
            transaction_type,
            amount,
            price,
        );
        Ok(transaction)
    }
} 