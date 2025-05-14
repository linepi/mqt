use fantoccini;
use tokio;

// 导入这些模块
mod models;  // 数据模型
mod tabs;    // 标签页类型
mod scraper; // 网页抓取
mod parser;  // 数据解析
mod io;      // 输入输出处理

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化WebDriver配置
    let caps = scraper::init_webdriver_config();
    
    // 启动ChromeDriver
    let mut chrome_driver = scraper::start_chromedriver()?;
    
    // 连接到WebDriver
    let client = fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:9516")
        .await?;
    
    // 打开TradingView筛选器页面
    client.goto("https://cn.tradingview.com/screener/").await?;
    println!("等待页面加载中...");
    
    // 加载所有数据
    // scraper::scroll_to_load_all(&client).await?;
    
    // 获取所有标签页的股票数据
    let stocks = scraper::fetch_stock_data(&client).await?;
    
    // 保存股票数据
    io::save_stock_data(&stocks)?;
    
    // 关闭浏览器和ChromeDriver
    client.close().await?;
    chrome_driver.kill()?;
    
    println!("数据抓取和保存完成。");
    Ok(())
}