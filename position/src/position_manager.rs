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
        let price = if response.status().is_success() {
            match response.json::<serde_json::Value>().await? {
                serde_json::Value::Number(n) => n.as_f64().ok_or("价格不是有效的数字")?,
                serde_json::Value::Object(_) => {
                    return Err("返回数据格式错误: map".into());
                },
                _ => return Err("返回数据格式错误".into())
            }
        } else {
            return Err(format!("获取价格失败: {}", response.status()).into());
        };

        let transaction = Transaction::new(
            code,
            transaction_type,
            amount,
            price,
        );
        Ok(transaction)
    }
} 