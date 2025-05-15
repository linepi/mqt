use actix_web::{web, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use log::info;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use strategy::models::{StrategyParams, StrategyType, BacktestResult};

// 策略管理状态
pub struct StrategyState {
    pub strategies: HashMap<String, StrategyParams>,
    pub backtest_results: HashMap<String, BacktestResult>,
}

impl StrategyState {
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        
        // 添加一些示例策略
        let momentum = StrategyParams::new("动量策略".to_string(), StrategyType::Momentum);
        strategies.insert(momentum.name.clone(), momentum);
        
        let mean_reversion = StrategyParams::new("均值回归策略".to_string(), StrategyType::MeanReversion);
        strategies.insert(mean_reversion.name.clone(), mean_reversion);
        
        Self {
            strategies,
            backtest_results: HashMap::new(),
        }
    }
}

// 策略请求
#[derive(Deserialize, Serialize)]
pub struct StrategyRequest {
    pub name: String,
}

// 回测请求
#[derive(Deserialize, Serialize)]
pub struct BacktestRequest {
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub initial_capital: Option<f64>,
}

// 获取所有策略
#[get("/list")]
pub async fn list_strategies(state: web::Data<Arc<Mutex<StrategyState>>>) -> impl Responder {
    info!("获取所有策略");
    
    let state = state.lock().unwrap();
    let strategies: Vec<&StrategyParams> = state.strategies.values().collect();
    
    HttpResponse::Ok().json(strategies)
}

// 获取策略详情
#[get("/detail/{name}")]
pub async fn get_strategy(
    state: web::Data<Arc<Mutex<StrategyState>>>,
    path: web::Path<String>,
) -> impl Responder {
    let name = path.into_inner();
    info!("获取策略详情: {}", name);
    
    let state = state.lock().unwrap();
    
    if let Some(strategy) = state.strategies.get(&name) {
        HttpResponse::Ok().json(strategy)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("未找到策略: {}", name)
        }))
    }
}

// 运行策略
#[post("/run")]
pub async fn run_strategy(
    state: web::Data<Arc<Mutex<StrategyState>>>,
    req: web::Json<StrategyRequest>,
) -> impl Responder {
    info!("运行策略: {}", req.name);
    
    let state = state.lock().unwrap();
    
    if let Some(strategy) = state.strategies.get(&req.name) {
        // 这里应该实际运行策略, 但目前只返回成功
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("策略 {} 已启动", req.name),
            "strategy": strategy
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("未找到策略: {}", req.name)
        }))
    }
}

// 回测策略
#[post("/backtest")]
pub async fn backtest_strategy(
    state: web::Data<Arc<Mutex<StrategyState>>>,
    req: web::Json<BacktestRequest>,
) -> impl Responder {
    info!("回测策略: {}", req.name);
    
    let mut state = state.lock().unwrap();
    
    if let Some(strategy) = state.strategies.get(&req.name) {
        // 克隆策略名称, 避免同时借用
        let strategy_name = strategy.name.clone();
        let strategy_type = strategy.strategy_type;
        
        // 这里应该实际执行回测, 但目前只返回模拟结果
        let result = BacktestResult {
            strategy_name: strategy_name.clone(),
            strategy_type,
            start_date: chrono::Utc::now() - chrono::Duration::days(30),
            end_date: chrono::Utc::now(),
            initial_capital: req.initial_capital.unwrap_or(100000.0),
            final_capital: 110000.0,
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
            trades: vec![],
        };
        
        // 保存回测结果
        state.backtest_results.insert(strategy_name, result.clone());
        
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("未找到策略: {}", req.name)
        }))
    }
}

// 获取回测结果
#[get("/backtest_result/{name}")]
pub async fn get_backtest_result(
    state: web::Data<Arc<Mutex<StrategyState>>>,
    path: web::Path<String>,
) -> impl Responder {
    let name = path.into_inner();
    info!("获取回测结果: {}", name);
    
    let state = state.lock().unwrap();
    
    if let Some(result) = state.backtest_results.get(&name) {
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("未找到策略 {} 的回测结果", name)
        }))
    }
}
