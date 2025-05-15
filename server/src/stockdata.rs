use actix_web::{web, HttpResponse, Responder, get, post};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex};
use std::process::Child;
use std::collections::HashMap;

// 定义模块状态
pub struct StockDataState {
    pub client: Option<fantoccini::Client>,
    pub chrome_driver: Option<Child>,
    pub is_fetching: bool,
    pub fetched_data: Vec<stockdata::models::StockData>,
    pub fetch_data_last_fetch: Option<chrono::DateTime<chrono::Utc>>,
    pub fetched_price: HashMap<String, f64>,
    pub fetched_price_last_fetch: Option<chrono::DateTime<chrono::Utc>>,
}

impl StockDataState {
    pub fn new() -> Self {
        Self {
            client: None,
            chrome_driver: None,
            is_fetching: false,
            fetch_data_last_fetch: None,
            fetched_data: Vec::new(),
            fetched_price: HashMap::new(),
            fetched_price_last_fetch: None,
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
#[post("/init")]
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
#[post("/close")]
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
#[post("/fetch")]
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
        state.fetch_data_last_fetch = Some(chrono::Utc::now());
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
#[get("/status")]
pub async fn get_stockdata_status(state: web::Data<Arc<Mutex<StockDataState>>>) -> impl Responder {
    info!("获取WebDriver状态");
    
    let state = state.lock().unwrap();
    
    let status = serde_json::json!({
        "initialized": state.client.is_some(),
        "last_fetch": state.fetch_data_last_fetch.map(|dt| dt.to_rfc3339()),
        "data_count": state.fetched_data.len()
    });
    
    HttpResponse::Ok().json(status)
}

#[get("/price")]
pub async fn get_price(state: web::Data<Arc<Mutex<StockDataState>>>, 
    web::Query(params): web::Query<HashMap<String, String>>) -> impl Responder {
    info!("获取股票价格");
    
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
        state.client.as_ref().unwrap().clone()
    };

    let mut state = state.lock().unwrap();

    if state.fetched_price.is_empty() || Utc::now().signed_duration_since(state.fetched_price_last_fetch.unwrap()) > chrono::Duration::seconds(60) {
        let price_map = stockdata::scraper::fetch_price(&client_option).await; 
        state.is_fetching = false;

        state.fetched_price = match price_map {
            Ok(map) => map,
            Err(e) => {
                error!("获取股票价格失败: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("获取股票价格失败: {}", e)
                }));
            }
        };

        state.fetched_price_last_fetch = Some(chrono::Utc::now());
    } 

    
    // 检查是否提供了code参数
    if let Some(code) = params.get("code") {
        if let Some(price) = state.fetched_price.get(code) {
            HttpResponse::Ok().json(price)
        } else {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("未找到代码为{}的股票价格", code)
            }))
        }
    } else {
        HttpResponse::Ok().json(&state.fetched_price)
    }
}

// 获取抓取的数据
#[get("/data")]
pub async fn get_stockdata(state: web::Data<Arc<Mutex<StockDataState>>>, 
    web::Query(params): web::Query<HashMap<String, String>>) -> impl Responder {
    info!("获取抓取的数据");
    
    let state = state.lock().unwrap();
    
    if state.fetched_data.is_empty() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "暂无数据, 请先调用/fetch接口抓取数据"
        }));
    }
    
    // 检查是否提供了code参数
    if let Some(code) = params.get("code") {
        // 如果提供了code参数，过滤匹配的数据
        let filtered_data: Vec<_> = state.fetched_data.iter()
            .filter(|stock| stock.code == *code)
            .collect();
            
        if filtered_data.is_empty() {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("未找到代码为{}的股票数据", code)
            }));
        }
        
        // 返回第一个匹配的数据（单个对象而不是数组）
        HttpResponse::Ok().json(filtered_data[0])
    } else {
        // 如果没有提供code参数，返回所有数据
        HttpResponse::Ok().json(&state.fetched_data)
    }
}
