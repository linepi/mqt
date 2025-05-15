use std::process::{Command, Child};
use fantoccini::Client;
use serde_json::{self, Value};
use crate::tabs::TabType;
use crate::models::StockData;
use crate::scripts;
use log::{info, error};

// 初始化WebDriver配置
pub fn init_webdriver_config() -> serde_json::map::Map<String, serde_json::Value> {
    let mut caps = serde_json::map::Map::new();
    
    // 添加Chrome选项
    let chrome_opts = serde_json::json!({
        "args": [
            "--headless",
            "--disable-gpu",
            "--no-sandbox",
            "--disable-dev-shm-usage",
            "--disable-web-security",
            "--window-size=1920,1080",
            "--disable-features=IsolateOrigins,site-per-process",
            "--disable-blink-features=AutomationControlled"
        ],
        "excludeSwitches": ["enable-automation"],
    });
    
    caps.insert("goog:chromeOptions".to_string(), chrome_opts);
    caps
}

// 启动ChromeDriver
pub fn start_chromedriver() -> Result<std::process::Child, std::io::Error> {
    Command::new("chromedriver")
        .arg("--port=9516")
        .spawn()
}

pub async fn create_webdriver_client() -> Result<(Child, Client), Box<dyn std::error::Error>> {
    // 初始化WebDriver配置
    let caps = init_webdriver_config();
    
    // 启动ChromeDriver
    let mut chrome_driver = match start_chromedriver() {
        Ok(driver) => driver,
        Err(e) => {
            let err_msg = format!("启动ChromeDriver失败: {}", e);
            error!("{}", err_msg);
            return Err(err_msg.into());
        }
    };
    
    // 连接到WebDriver - 简化此处代码, 移除临时变量
    let client = match 
        fantoccini::ClientBuilder::native().capabilities(caps).connect("http://localhost:9516").await {
        Ok(client) => client,
        Err(e) => {
            let _ = chrome_driver.kill();
            let err_msg = format!("连接到WebDriver失败: {}", e);
            error!("{}", err_msg);
            return Err(err_msg.into());
        }
    };
    
    Ok((chrome_driver, client))
}

pub async fn scroll_to_load_all(client: &fantoccini::Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_count = 0;
    let mut noupdate_times = 0;
    loop {
        client.execute(
            scripts::get_scroll_script(),
            vec![],
        ).await?;
        let js_result = client.execute(
            scripts::get_row_count_script(),
            vec![],
        ).await?;
        let count = js_result.as_i64().unwrap_or(0);
        if count == last_count || count == 0 {
            noupdate_times += 1;
            if noupdate_times > 30 {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        } else {
            noupdate_times = 0;
            last_count = count;
        }
    }
    Ok(())
}

// interval: 间隔时间, 单位：毫秒
// timeout: 超时时间, 单位：毫秒
pub async fn wait_until_script_return_true(client: &Client, script: &str, interval: u64, timeout: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = false;
    let start_time = std::time::Instant::now();
    while !result {
        let js_result = client.execute(script, vec![]).await?;
        result = js_result.as_bool().unwrap_or(false);
        tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
        if start_time.elapsed() > std::time::Duration::from_millis(timeout) {
            return Err(format!("等待脚本返回true超时, 脚本: {}, 间隔: {}ms, 超时: {}ms", script, interval, timeout).into());
        }
    }
    Ok(())
}

// 切换到指定标签页
pub async fn switch_to_tab(client: &Client, tab: TabType) -> Result<(), Box<dyn std::error::Error>> {
    info!("切换到{}标签页...", tab.name());
    
    let js_click = scripts::get_tab_click_script(tab.id());
    
    let clicked = client.execute(&js_click, vec![]).await?;
    
    if !clicked.as_bool().unwrap_or(false) {
        return Err(format!("找不到{}标签页", tab.name()).into());
    }
    
    // 等待标签页加载完成
    wait_until_script_return_true(
        client, 
        scripts::get_tab_loaded_check_script(), 
        200, 10000).await?;
    
    Ok(())
}

// 使用JavaScript执行数据抓取, 获取所有标签页的股票数据
pub async fn fetch_stock_data(client: &Client) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    info!("开始从所有标签页获取股票数据...");
    
    // 存储每个标签页的数据集合
    let mut tab_data_sources = Vec::new();
    
    // 获取所有标签页的数据
    for tab in TabType::all().iter() {
        match fetch_stock_data_from_tab(client, *tab).await {
            Ok(tab_stocks) => {
                info!("成功获取{}标签页数据: {}支股票", tab.name(), tab_stocks.len());
                tab_data_sources.push(tab_stocks);
            },
            Err(e) => {
                error!("获取{}标签页数据失败: {}", tab.name(), e);
            }
        }
    }
    
    // 使用merge_stock_data_sources函数合并所有标签页的数据
    let all_stock_data = crate::io::merge_stock_data_sources(&tab_data_sources);
    
    info!("成功获取并合并{}支股票的数据", all_stock_data.len());
    Ok(all_stock_data)
}

// 从指定标签页获取股票数据
pub async fn fetch_stock_data_from_tab(client: &Client, tab: TabType) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    // 切换到指定标签页
    switch_to_tab(client, tab).await?;
    
    // 执行JavaScript获取实际内容
    info!("正在从{}标签页获取数据...", tab.name());
    
    // 获取原始JavaScript执行结果
    let js_result = client.execute(
        &scripts::get_data_extraction_script(&tab),
        vec![],
    ).await?;
    
    // 将JavaScript结果转换为JSON值
    let json_str = js_result.as_str().unwrap_or("[]");
    let json_data: Value = serde_json::from_str(json_str)?;
    
    // 使用解析器将JSON值转换为StockData对象
    let stocks = crate::parser::parse_stock_data_from_json(json_data, tab)?;
    
    info!("已从{}标签页获取{}支股票的数据", tab.name(), stocks.len());
    Ok(stocks)
} 

pub async fn perform_fetch(
    client: Client, 
    save_to_file: bool
) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    // 打开TradingView筛选器页面
    match client.goto("https://cn.tradingview.com/screener/").await {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("打开TradingView页面失败: {}", e).into());
        }
    }
    
    // 等待页面加载完成
    match wait_until_script_return_true(
        &client, 
        scripts::get_page_loaded_check_script(), 
        200, 
        10000
    ).await {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("等待页面加载失败: {}", e.to_string()).into());
        }
    }

    match scroll_to_load_all(&client).await {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("滚动加载失败: {}", e.to_string()).into());
        }
    }
    
    // 获取所有标签页的股票数据
    let stocks = match fetch_stock_data(&client).await {
        Ok(stocks) => stocks,
        Err(e) => {
            return Err(format!("获取股票数据失败: {}", e.to_string()).into());
        }
    };
    
    // 如果需要保存到文件
    if save_to_file {
        if let Err(e) = crate::io::save_stock_data(&stocks) {
            error!("保存数据到文件失败: {}", e);
        } else {
            info!("数据已保存到文件");
        }
    }
    
    Ok(stocks)
}