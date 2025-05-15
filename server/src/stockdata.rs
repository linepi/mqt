use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex};
use std::process::Child;

// 定义模块状态
pub struct StockDataState {
    pub client: Option<fantoccini::Client>,
    pub chrome_driver: Option<Child>,
    pub is_fetching: bool,
    pub last_fetch: Option<chrono::DateTime<chrono::Utc>>,
    pub fetched_data: Vec<stockdata::models::StockData>,
}

impl StockDataState {
    pub fn new() -> Self {
        Self {
            client: None,
            chrome_driver: None,
            is_fetching: false,
            last_fetch: None,
            fetched_data: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FetchRequest {
    pub save_to_file: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FetchResponse {
    pub success: bool,
    pub message: String,
    pub count: usize,
}

// 初始化WebDriver
pub async fn init_webdriver(state: web::Data<Arc<Mutex<StockDataState>>>) -> impl Responder {
    {
        let state = state.lock().unwrap();
        if state.chrome_driver.is_some() || state.client.is_some() {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "message": "WebDriver已经初始化"
            }));
        }
    }

    info!("初始化WebDriver");
    
    let mut state = state.lock().unwrap();
    
    // 初始化新的WebDriver
    match stockdata::scraper::create_webdriver_client().await {
        Ok((chrome_driver, client)) => {
            state.client = Some(client);
            state.chrome_driver = Some(chrome_driver);
            info!("WebDriver初始化成功");
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "WebDriver初始化成功"
            }))
        },
        Err(e) => {
            error!("WebDriver初始化失败: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("WebDriver初始化失败: {}", e)
            }))
        }
    }
}

// 关闭WebDriver
pub async fn close_webdriver(state: web::Data<Arc<Mutex<StockDataState>>>) -> impl Responder {
    let mut state = state.lock().unwrap();
    
    if state.is_fetching {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "数据抓取正在进行中, 无法关闭WebDriver"
        }));
    }

    if state.client.is_none() && state.chrome_driver.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "WebDriver未初始化"
        }));
    }
    
    let mut success = true;
    let mut message = "WebDriver已关闭".to_string();
    
    // 关闭浏览器
    if let Some(client) = state.client.take() {
        match client.close().await {
            Ok(_) => {},
            Err(e) => {
                success = false;
                message = format!("关闭浏览器失败: {}", e);
                error!("{}", message);
            }
        }
    }
    
    // 关闭ChromeDriver
    if let Some(mut chrome_driver) = state.chrome_driver.take() {
        match chrome_driver.kill() {
            Ok(_) => {},
            Err(e) => {
                success = false;
                message = format!("关闭ChromeDriver失败: {}", e);
                error!("{}", message);
            }
        }
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": success,
        "message": message
    }))
}

// 抓取股票数据
pub async fn fetch_data(state: web::Data<Arc<Mutex<StockDataState>>>, req: web::Json<FetchRequest>) -> impl Responder {
    // 检查是否可以开始抓取
    let client_option = {
        let mut state = state.lock().unwrap();
        
        if state.is_fetching {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "数据抓取正在进行中"
            }));
        }
        
        if state.client.is_none() {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "WebDriver未初始化, 请先调用init接口"
            }));
        }
        
        state.is_fetching = true;
        state.fetched_data.clear();
        state.last_fetch = Some(chrono::Utc::now());
        state.client.as_ref().unwrap().clone()
    };
    
    let save_to_file = req.save_to_file.unwrap_or(false);
    let state_clone = Arc::clone(&state);
    
    // 启动异步任务执行数据抓取
    actix_web::rt::spawn(async move {
        info!("开始抓取数据...");
        
        let fetch_result = 
            stockdata::scraper::perform_fetch(client_option, save_to_file).await;
        // 处理结果
        match fetch_result {
            Ok(stocks) => {
                info!("成功获取{}支股票的数据", stocks.len());
                
                // 更新状态
                let mut state = state_clone.lock().unwrap();
                state.fetched_data = stocks;
                state.is_fetching = false;
            },
            Err(e) => {
                error!("{}", e);
                let mut state = state_clone.lock().unwrap();
                state.is_fetching = false;
            }
        }
    });
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "数据抓取任务已启动",
        "count": 0
    }))
}

// 获取WebDriver状态
pub async fn get_stockdata_status(state: web::Data<Arc<Mutex<StockDataState>>>) -> impl Responder {
    info!("获取WebDriver状态");
    
    let state = state.lock().unwrap();
    
    let status = serde_json::json!({
        "initialized": state.client.is_some(),
        "last_fetch": state.last_fetch.map(|dt| dt.to_rfc3339()),
        "data_count": state.fetched_data.len()
    });
    
    HttpResponse::Ok().json(status)
}

// 获取抓取的数据
pub async fn get_stockdata(state: web::Data<Arc<Mutex<StockDataState>>>) -> impl Responder {
    info!("获取抓取的数据");
    
    let state = state.lock().unwrap();
    
    if state.fetched_data.is_empty() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "暂无数据, 请先调用/fetch接口抓取数据"
        }));
    }
    
    HttpResponse::Ok().json(&state.fetched_data)
}
