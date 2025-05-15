use actix_web::{web, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use position::models::{Portfolio, Position, Transaction, TransactionType};

// 持仓管理状态
pub struct PositionState {
    pub portfolio: Portfolio,
}

impl PositionState {
    pub fn new() -> Self {
        Self {
            portfolio: Portfolio::new("默认组合".to_string(), 100000.0),
        }
    }
}

// 添加持仓请求
#[derive(Deserialize, Serialize)]
pub struct AddPositionRequest {
    pub code: String,
    pub name: Option<String>,
    pub amount: f64,
    pub price: f64,
}

// 交易请求
#[derive(Deserialize, Serialize)]
pub struct TransactionRequest {
    pub code: String,
    pub amount: f64,
    pub price: Option<f64>,
}

// 获取所有持仓
#[get("/list")]
pub async fn list_positions(state: web::Data<Arc<Mutex<PositionState>>>) -> impl Responder {
    info!("获取所有持仓");
    
    let state = state.lock().unwrap();
    let positions: Vec<&Position> = state.portfolio.positions.values().collect();
    
    HttpResponse::Ok().json(positions)
}

// 获取投资组合信息
#[get("/portfolio")]
pub async fn get_portfolio(state: web::Data<Arc<Mutex<PositionState>>>) -> impl Responder {
    info!("获取投资组合信息");
    
    let state = state.lock().unwrap();
    
    HttpResponse::Ok().json(&state.portfolio)
}

// 添加持仓
#[post("/add")]
pub async fn add_position(
    state: web::Data<Arc<Mutex<PositionState>>>,
    req: web::Json<AddPositionRequest>,
) -> impl Responder {
    info!("添加持仓: {} x {}", req.code, req.amount);
    
    let mut state = state.lock().unwrap();
    
    // 创建交易记录
    let transaction = Transaction::new(
        req.code.clone(),
        TransactionType::Buy,
        req.amount,
        req.price,
    );
    
    // 添加交易到投资组合
    match state.portfolio.add_transaction(transaction) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "持仓添加成功"
            }))
        },
        Err(e) => {
            error!("添加持仓失败: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

// 减少持仓
#[post("/remove")]
pub async fn remove_position(
    state: web::Data<Arc<Mutex<PositionState>>>,
    req: web::Json<TransactionRequest>,
) -> impl Responder {
    info!("减少持仓: {} x {}", req.code, req.amount);
    
    let mut state = state.lock().unwrap();
    
    // 获取当前价格, 如果没有提供价格
    let price = if let Some(price) = req.price {
        price
    } else {
        // 从持仓中获取价格
        if let Some(position) = state.portfolio.positions.get(&req.code) {
            if let Some(current_price) = position.current_price {
                current_price
            } else {
                position.cost // 如果没有当前价格, 使用成本价
            }
        } else {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "没有该股票的持仓"
            }));
        }
    };
    
    // 创建交易记录
    let transaction = Transaction::new(
        req.code.clone(),
        TransactionType::Sell,
        req.amount,
        price,
    );
    
    // 添加交易到投资组合
    match state.portfolio.add_transaction(transaction) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "持仓减少成功"
            }))
        },
        Err(e) => {
            error!("减少持仓失败: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

// 更新持仓价格
#[post("/update_prices")]
pub async fn update_prices(
    state: web::Data<Arc<Mutex<PositionState>>>,
    prices: web::Json<HashMap<String, f64>>,
) -> impl Responder {
    info!("更新持仓价格");
    
    let mut state = state.lock().unwrap();
    
    // 更新价格
    for (code, price) in prices.iter() {
        if let Some(position) = state.portfolio.positions.get_mut(code) {
            position.current_price = Some(*price);
            position.last_update = chrono::Utc::now();
        }
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "价格更新成功"
    }))
}