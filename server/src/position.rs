use actix_web::{web, HttpResponse, Responder, get, post};
use position::position_manager::PositionManager;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex};
use position::models::{Portfolio, TransactionType};

// 持仓管理状态
pub struct PositionState {
    pub portfolios: Vec<Portfolio>,
}

impl PositionState {
    pub fn new() -> Self {
        Self {
            portfolios: vec![],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddPortfolioRequest {
    pub name: String,
    pub cash_balance: f64,
}

#[derive(Deserialize, Serialize)]
pub struct RemovePortfolioRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddPositionRequest {
    pub portfolio: String,
    pub code: String,
    pub amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct RemovePositionRequest {
    pub portfolio: String,
    pub code: String,
    pub amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct QueryPortfolioRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct QueryPortfolioResponse {
    pub name: String,
    pub cash_balance: f64,
    pub positions: Vec<position::models::Position>,
}

// 获取所有持仓
#[get("/list")]
pub async fn list_positions(state: web::Data<Arc<Mutex<PositionState>>>) -> impl Responder {
    info!("获取所有持仓");
    
    let state = state.lock().unwrap();
    let names: Vec<String> = state.portfolios.iter().map(|p| p.name.clone()).collect();
    
    HttpResponse::Ok().json(names)
}

// 获取投资组合信息
#[post("/query_portfolio")]
pub async fn get_portfolio(state: web::Data<Arc<Mutex<PositionState>>>, req: web::Json<QueryPortfolioRequest>) -> impl Responder {
    info!("获取投资组合信息");
    
    let state = state.lock().unwrap();

    match state.portfolios.iter().find(|p| p.name == req.name) {
        Some(portfolio) => HttpResponse::Ok().json(QueryPortfolioResponse {
            name: portfolio.name.clone(),
            cash_balance: portfolio.cash_balance,
            positions: portfolio.positions.values().cloned().collect(),
        }),
        None => HttpResponse::BadRequest().json(serde_json::json!({ "error": "投资组合不存在" }))
    }
}

#[post("/add_portfolio")]
pub async fn add_portfolio(state: web::Data<Arc<Mutex<PositionState>>>, req: web::Json<AddPortfolioRequest>) -> impl Responder {
    info!("添加投资组合: {}", req.name);
    
    let mut state = state.lock().unwrap();

    // 检查投资组合是否已存在
    if state.portfolios.iter().any(|p| p.name == req.name) {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "投资组合已存在" }));
    }

    // 添加新的投资组合
    state.portfolios.push(Portfolio::new(req.name.clone(), req.cash_balance));
    HttpResponse::Ok().json(serde_json::json!({ "success": true }))
}

#[post("/remove_portfolio")]
pub async fn remove_portfolio(state: web::Data<Arc<Mutex<PositionState>>>, req: web::Json<RemovePortfolioRequest>) -> impl Responder {
    info!("删除投资组合: {}", req.name);

    let mut state = state.lock().unwrap();

    // 检查投资组合是否存在
    if !state.portfolios.iter().any(|p| p.name == req.name) {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "投资组合不存在" }));
    }

    // 删除投资组合
    state.portfolios.retain(|p| p.name != req.name);
    HttpResponse::Ok().json(serde_json::json!({ "success": true }))
}

// 添加持仓
#[post("/add")]
pub async fn add_position(
    state: web::Data<Arc<Mutex<PositionState>>>,
    req: web::Json<AddPositionRequest>,
) -> impl Responder {
    info!("添加持仓: portfolio: {}, code: {}, amount: {}", req.portfolio, req.code, req.amount);
    
    let mut state = state.lock().unwrap();

    let portfolio = state.portfolios.iter_mut().find(|p| p.name == req.portfolio);
    // 检查投资组合是否存在
    if portfolio.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "投资组合不存在" }));
    }
    let portfolio = portfolio.unwrap();

    let mut position_manager = PositionManager::new(portfolio);

    let transaction = match position_manager.new_transaction(req.code.clone(), TransactionType::Buy, req.amount).await {
        Ok(transaction) => transaction,
        Err(e) => {
            error!("添加持仓失败: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() }));
        }
    };
    
    // 添加交易到投资组合
    match portfolio.add_transaction(transaction) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "持仓添加成功"
            }))
        },
        Err(e) => {
            error!("添加交易到投资组合失败: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

// 减少持仓
#[post("/remove")]
pub async fn remove_position(
    state: web::Data<Arc<Mutex<PositionState>>>,
    req: web::Json<RemovePositionRequest>,
) -> impl Responder {
    info!("减少持仓: portfolio: {}, code: {}, amount: {}", req.portfolio, req.code, req.amount);
    
    let mut state = state.lock().unwrap();

    // 检查投资组合是否存在
    let portfolio = state.portfolios.iter_mut().find(|p| p.name == req.portfolio);
    if portfolio.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "投资组合不存在" }));
    }
    let portfolio = portfolio.unwrap();

    let mut position_manager = PositionManager::new(portfolio);

    let transaction = match position_manager.new_transaction(req.code.clone(), TransactionType::Sell, req.amount).await {
        Ok(transaction) => transaction,
        Err(e) => {
            error!("减少持仓失败: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() }));
        }
    };
    
    // 添加交易到投资组合
    match portfolio.add_transaction(transaction) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "持仓减少成功"
            }))
        },
        Err(e) => {
            error!("减少持仓失败: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}
