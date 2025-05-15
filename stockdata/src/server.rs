use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::process::Child;
use fantoccini::Client;
use log::{info, error};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;

use crate::models::StockData;
use crate::scraper;
use crate::scripts;
use crate::io;

// 自定义错误类型，实现Send和Sync
#[derive(Debug)]
pub struct ServerError(String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for ServerError {}

// 服务器状态
pub struct AppState {
    chrome_driver: Option<Child>,
    client: Option<Client>,
    stocks: Vec<StockData>,
    is_running: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            chrome_driver: None,
            client: None,
            stocks: Vec::new(),
            is_running: false,
        }
    }
}

// 初始化响应
#[derive(Serialize)]
struct InitResponse {
    success: bool,
    message: String,
}

// 抓取数据请求
#[derive(Deserialize)]
struct FetchRequest {
    save_to_file: Option<bool>,
}

// 抓取数据响应
#[derive(Serialize)]
struct FetchResponse {
    success: bool,
    message: String,
    count: usize,
}

// 状态响应
#[derive(Serialize)]
struct StatusResponse {
    is_running: bool,
    stocks_count: usize,
}

// 创建WebDriver客户端并返回结果
async fn create_webdriver_client() -> Result<(Child, Client), ServerError> {
    // 初始化WebDriver配置
    let caps = scraper::init_webdriver_config();
    
    // 启动ChromeDriver
    let mut chrome_driver = match scraper::start_chromedriver() {
        Ok(driver) => driver,
        Err(e) => {
            let err_msg = format!("启动ChromeDriver失败: {}", e);
            error!("{}", err_msg);
            return Err(ServerError(err_msg));
        }
    };
    
    // 连接到WebDriver - 简化此处代码，移除临时变量
    let client = match 
        fantoccini::ClientBuilder::native().capabilities(caps).connect("http://localhost:9516").await {
        Ok(client) => client,
        Err(e) => {
            let _ = chrome_driver.kill();
            let err_msg = format!("连接到WebDriver失败: {}", e);
            error!("{}", err_msg);
            return Err(ServerError(err_msg));
        }
    };
    
    Ok((chrome_driver, client))
}

// 初始化WebDriver
async fn init_webdriver(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    // 检查WebDriver是否已初始化
    {
        let state = data.lock().unwrap();
        if state.chrome_driver.is_some() || state.client.is_some() {
            return HttpResponse::BadRequest().json(InitResponse {
                success: false,
                message: "WebDriver已经初始化".to_string(),
            });
        }
    }
    
    // 创建WebDriver客户端
    let webdriver_result = create_webdriver_client().await;
    
    match webdriver_result {
        Ok((chrome_driver, client)) => {
            // 更新状态
            let mut state = data.lock().unwrap();
            state.chrome_driver = Some(chrome_driver);
            state.client = Some(client);
            
            HttpResponse::Ok().json(InitResponse {
                success: true,
                message: "WebDriver初始化成功".to_string(),
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(InitResponse {
                success: false,
                message: e.to_string(),
            })
        }
    }
}

// 关闭WebDriver
async fn close_webdriver(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let mut state = data.lock().unwrap();
    
    if state.is_running {
        return HttpResponse::BadRequest().json(InitResponse {
            success: false,
            message: "数据抓取正在进行中，无法关闭WebDriver".to_string(),
        });
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
    
    HttpResponse::Ok().json(InitResponse {
        success,
        message,
    })
}

// 执行数据抓取的函数，返回可发送的错误类型
async fn perform_fetch(
    client: Client, 
    save_to_file: bool, 
    _state: Arc<Mutex<AppState>>  // 添加下划线前缀表示intentionally unused
) -> Result<Vec<StockData>, ServerError> {
    // 打开TradingView筛选器页面
    match client.goto("https://cn.tradingview.com/screener/").await {
        Ok(_) => {},
        Err(e) => {
            return Err(ServerError(format!("打开TradingView页面失败: {}", e)));
        }
    }
    
    // 等待页面加载完成
    match scraper::wait_until_script_return_true(
        &client, 
        scripts::get_page_loaded_check_script(), 
        200, 
        10000
    ).await {
        Ok(_) => {},
        Err(e) => {
            return Err(ServerError(format!("等待页面加载失败: {}", e.to_string())));
        }
    }

    match scraper::scroll_to_load_all(&client).await {
        Ok(_) => {},
        Err(e) => {
            return Err(ServerError(format!("滚动加载失败: {}", e.to_string())));
        }
    }
    
    // 获取所有标签页的股票数据
    let stocks = match scraper::fetch_stock_data(&client).await {
        Ok(stocks) => stocks,
        Err(e) => {
            return Err(ServerError(format!("获取股票数据失败: {}", e.to_string())));
        }
    };
    
    // 如果需要保存到文件
    if save_to_file {
        if let Err(e) = io::save_stock_data(&stocks) {
            error!("保存数据到文件失败: {}", e);
        } else {
            info!("数据已保存到文件");
        }
    }
    
    Ok(stocks)
}

// 开始抓取数据
async fn fetch_data(data: web::Data<Arc<Mutex<AppState>>>, req: web::Json<FetchRequest>) -> impl Responder {
    // 检查是否可以开始抓取
    let client_option = {
        let mut state = data.lock().unwrap();
        
        if state.is_running {
            return HttpResponse::BadRequest().json(FetchResponse {
                success: false,
                message: "数据抓取正在进行中".to_string(),
                count: 0,
            });
        }
        
        if state.client.is_none() {
            return HttpResponse::BadRequest().json(FetchResponse {
                success: false,
                message: "WebDriver未初始化，请先调用init接口".to_string(),
                count: 0,
            });
        }
        
        state.is_running = true;
        state.stocks.clear();
        
        state.client.as_ref().unwrap().clone()
    };
    
    let save_to_file = req.save_to_file.unwrap_or(false);
    let state_clone = Arc::clone(&data);
    
    // 启动异步任务执行数据抓取
    actix_web::rt::spawn(async move {
        info!("开始抓取数据...");
        
        // 执行抓取
        let fetch_result = perform_fetch(client_option, save_to_file, Arc::clone(&state_clone)).await;
        
        // 处理结果
        match fetch_result {
            Ok(stocks) => {
                info!("成功获取{}支股票的数据", stocks.len());
                
                // 更新状态
                let mut state = state_clone.lock().unwrap();
                state.stocks = stocks;
                state.is_running = false;
            },
            Err(e) => {
                error!("{}", e);
                let mut state = state_clone.lock().unwrap();
                state.is_running = false;
            }
        }
    });
    
    HttpResponse::Ok().json(FetchResponse {
        success: true,
        message: "数据抓取任务已启动".to_string(),
        count: 0,
    })
}

// 获取状态
async fn get_status(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().unwrap();
    
    HttpResponse::Ok().json(StatusResponse {
        is_running: state.is_running,
        stocks_count: state.stocks.len(),
    })
}

// 获取数据
async fn get_data(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().unwrap();
    
    if state.stocks.is_empty() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "message": "没有可用的股票数据，请先抓取数据"
        }));
    }
    
    HttpResponse::Ok().json(&state.stocks)
}

// 配置和启动HTTP服务器
pub async fn start_server(port: u16) -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(AppState::new()));
    
    info!("启动HTTP服务器，端口: {}", port);
    
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("LANG", "zh_CN.UTF-8");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Arc::clone(&state)))
            .route("/api/init", web::post().to(init_webdriver))
            .route("/api/close", web::post().to(close_webdriver))
            .route("/api/fetch", web::post().to(fetch_data))
            .route("/api/status", web::get().to(get_status))
            .route("/api/data", web::get().to(get_data))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
} 