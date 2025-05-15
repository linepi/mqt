use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use log::info;
use std::sync::{Arc, Mutex};

mod server;
mod position;
mod strategy;
mod stockdata;

// 导入相关函数
use crate::stockdata::{init_webdriver, close_webdriver, fetch_data, get_stockdata_status, get_stockdata, StockDataState};
use crate::position::{list_positions, get_portfolio, add_position, remove_position, update_prices, PositionState};
use crate::strategy::{list_strategies, get_strategy, run_strategy, backtest_strategy, get_backtest_result, StrategyState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置环境变量
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("LANG", "zh_CN.UTF-8");
    
    // 初始化日志系统
    env_logger::init();
    
    info!("启动交易系统服务器...");
    
    // 创建服务器状态
    let server_state = web::Data::new(server::ServerState::new());
    
    // 创建各子模块状态
    let stockdata_state = Arc::new(Mutex::new(StockDataState::new()));
    let position_state = Arc::new(Mutex::new(PositionState::new()));
    let strategy_state = Arc::new(Mutex::new(StrategyState::new()));
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .app_data(server_state.clone())
            // 注册服务器状态API
            .route("/api/status", web::get().to(server::get_status))
            .route("/api/health", web::get().to(server::health_check))
            // 注册股票数据模块API
            .service(
                web::scope("/api/stockdata")
                    .app_data(web::Data::new(stockdata_state.clone()))
                    .route("/init", web::post().to(init_webdriver))
                    .route("/close", web::post().to(close_webdriver))
                    .route("/fetch", web::post().to(fetch_data))
                    .route("/status", web::get().to(get_stockdata_status))
                    .route("/data", web::get().to(get_stockdata))
            )
            // 注册仓位管理模块API
            .service(
                web::scope("/api/position")
                    .app_data(web::Data::new(position_state.clone()))
                    .service(list_positions)
                    .service(get_portfolio)
                    .service(add_position)
                    .service(remove_position)
                    .service(update_prices)
            )
            // 注册策略模块API
            .service(
                web::scope("/api/strategy")
                    .app_data(web::Data::new(strategy_state.clone()))
                    .service(list_strategies)
                    .service(get_strategy)
                    .service(run_strategy)
                    .service(backtest_strategy)
                    .service(get_backtest_result)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 