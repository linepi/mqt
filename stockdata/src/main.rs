use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use fantoccini;

// 定义股票数据结构
#[derive(Debug, Serialize, Deserialize)]
struct StockData {
    code: String,
    name: String,
    price: f64,
    change_percent: String,
    volume: String,
    reletive_volume: f64,
    market_cap: String,
    pe_ratio: f64,
    eps: String,
    earnings_growth: String,
    dividend_yield: String,
    sector: String,
    rating: String,
}

impl StockData {
    fn new() -> Self {
        StockData {
            code: String::new(),
            name: String::new(),
            price: 0.0,
            change_percent: String::new(),
            volume: String::new(),
            reletive_volume: 0.0,
            market_cap: String::new(),
            pe_ratio: 0.0,
            eps: String::new(),
            earnings_growth: String::new(),
            dividend_yield: String::new(),
            sector: String::new(),
            rating: String::new(),
        }
    }
    
    fn display(&self) {
        println!("股票代码: {}", self.code);
        println!("股票名称: {}", self.name);
        println!("当前价格: {:.2}", self.price);
        println!("涨跌幅: {}", self.change_percent);
        println!("成交量: {}", self.volume);
        println!("相对成交量: {:.2}", self.reletive_volume);
        println!("市值: {}", self.market_cap);
        println!("市盈率: {:.2}", self.pe_ratio);
        println!("每股收益: {}", self.eps);
        println!("盈利增长: {}", self.earnings_growth);
        println!("股息率: {}", self.dividend_yield);
        println!("行业: {}", self.sector);
        println!("评级: {}", self.rating);
        println!("------------------------");
    }
}

// 初始化WebDriver配置
fn init_webdriver_config() -> serde_json::map::Map<String, serde_json::Value> {
    let mut caps = serde_json::map::Map::new();
    let mut opts = serde_json::map::Map::new();
    opts.insert("args".to_string(), serde_json::json!(["--headless", "--disable-gpu", "--no-sandbox"]));
    // opts.insert("args".to_string(), serde_json::json!([
    //     "--disable-gpu", 
    //     "--no-sandbox", 
    //     "--window-size=1920,1080",
    //     "--ignore-certificate-errors",  // 添加忽略证书错误
    //     "--disable-extensions",         // 禁用扩展
    //     "--disable-web-security"        // 禁用网页安全性检查
    // ]));
    caps.insert("goog:chromeOptions".to_string(), serde_json::json!(opts));
    caps
}

// 启动ChromeDriver
fn start_chromedriver() -> Result<std::process::Child, std::io::Error> {
    std::process::Command::new("chromedriver")
        .arg("--port=9516")
        .spawn()
}

// 自动滚动页面，直到所有数据加载完成
async fn scroll_to_load_all(client: &fantoccini::Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_count = 0;
    let mut noupdate_times = 0;
    loop {
        client.execute(
            r#"
            var containers = [
                document.querySelector('.wrapper-TlN8oopI'),
                document.querySelector('.isOverflowHidden-OuXcFHzP'),
                document.querySelector('#js-screener-container div.wrapper-TlN8oopI'),
                document.querySelector('#js-screener-container > div.root-wANuhlbW > div > div.isOverflowHidden-OuXcFHzP > div > div > div.wrapper-TlN8oopI')
            ];
            
            for(let i=0; i<containers.length; i++) {
                if(containers[i]) {
                    containers[i].scrollTop = containers[i].scrollHeight;
                    return;
                }
            }
            "#,
            vec![],
        ).await?;
        let js_result = client.execute(
            "return document.querySelectorAll('tbody[tabindex=\"100\"] tr').length;",
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

// 使用JavaScript获取股票数据（去掉20的限制，获取全部）
async fn fetch_stock_data_with_js(client: &fantoccini::Client) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    // 执行JavaScript获取实际内容
    println!("执行JavaScript获取数据...");
    let js_result = client.execute(
        r#"
        const rows = document.querySelectorAll('tbody[tabindex="100"] tr');
        const result = [];
        for (let i = 0; i < rows.length; i++) {
            const cells = rows[i].querySelectorAll('td');
            const rowData = {};
            if (cells.length > 0) {
                // 股票代码和名称
                const tickerCell = cells[0];
                const tickerLink = tickerCell.querySelector('a.tickerName-GrtoTeat');
                if (tickerLink) {
                    rowData.code = tickerLink.textContent.trim();
                    rowData.name = tickerCell.querySelector('sup.tickerDescription-GrtoTeat').textContent.trim();
                }
                // 价格
                if (cells[1]) {
                    const priceText = cells[1].textContent.replace('CNY', '').trim();
                    rowData.price = priceText.replace(',', '');
                }
                // 涨跌幅
                if (cells[2]) {
                    rowData.changePercent = cells[2].textContent.trim();
                }
                // 成交量
                if (cells[3]) {
                    rowData.volume = cells[3].textContent.trim();
                }
                // 相对成交量
                if (cells[4]) {
                    rowData.relativeVolume = cells[4].textContent.trim();
                }
                // 市值
                if (cells[5]) {
                    rowData.marketCap = cells[5].textContent.trim();
                }
                // PE比率
                if (cells[6]) {
                    rowData.peRatio = cells[6].textContent.trim();
                }
                // EPS
                if (cells[7]) {
                    rowData.eps = cells[7].textContent.trim();
                }
                // 盈利增长
                if (cells[8]) {
                    rowData.earningsGrowth = cells[8].textContent.trim();
                }
                // 股息率
                if (cells[9]) {
                    rowData.dividendYield = cells[9].textContent.trim();
                }
                // 行业
                if (cells[10]) {
                    const sectorLink = cells[10].querySelector('a');
                    if (sectorLink) {
                        rowData.sector = sectorLink.textContent.trim();
                    }
                }
                // 评级
                if (cells[11]) {
                    rowData.rating = cells[11].textContent.trim();
                }
                result.push(rowData);
            }
        }
        return JSON.stringify(result);
        "#,
        vec![],
    ).await?;
    // 解析JavaScript结果
    let json_str = js_result.as_str().unwrap_or("[]");
    let js_data: serde_json::Value = serde_json::from_str(json_str)?;
    println!("成功获取数据，正在处理...");
    parse_stock_data_from_json(js_data)
}

// 从JSON解析股票数据
fn parse_stock_data_from_json(js_data: serde_json::Value) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    let mut stocks: Vec<StockData> = Vec::new();
    
    if let serde_json::Value::Array(items) = js_data {
        for item in items {
            let mut stock = StockData::new();
            
            if let Some(code) = item.get("code").and_then(|v| v.as_str()) {
                stock.code = code.to_string();
            }
            
            if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                stock.name = name.to_string();
            }
            
            if let Some(price) = item.get("price").and_then(|v| v.as_str()) {
                stock.price = price.parse::<f64>().unwrap_or(0.0);
            }
            
            if let Some(change_percent) = item.get("changePercent").and_then(|v| v.as_str()) {
                stock.change_percent = change_percent.to_string();
            }
            
            if let Some(volume) = item.get("volume").and_then(|v| v.as_str()) {
                stock.volume = volume.to_string();
            }
            
            if let Some(relative_volume) = item.get("relativeVolume").and_then(|v| v.as_str()) {
                stock.reletive_volume = relative_volume.parse::<f64>().unwrap_or(0.0);
            }
            
            if let Some(market_cap) = item.get("marketCap").and_then(|v| v.as_str()) {
                stock.market_cap = market_cap.to_string();
            }
            
            if let Some(pe_ratio) = item.get("peRatio").and_then(|v| v.as_str()) {
                stock.pe_ratio = pe_ratio.parse::<f64>().unwrap_or(0.0);
            }
            
            if let Some(eps) = item.get("eps").and_then(|v| v.as_str()) {
                stock.eps = eps.to_string();
            }
            
            if let Some(earnings_growth) = item.get("earningsGrowth").and_then(|v| v.as_str()) {
                stock.earnings_growth = earnings_growth.to_string();
            }
            
            if let Some(dividend_yield) = item.get("dividendYield").and_then(|v| v.as_str()) {
                stock.dividend_yield = dividend_yield.to_string();
            }
            
            if let Some(sector) = item.get("sector").and_then(|v| v.as_str()) {
                stock.sector = sector.to_string();
            }
            
            if let Some(rating) = item.get("rating").and_then(|v| v.as_str()) {
                stock.rating = rating.to_string();
            }
            
            stocks.push(stock);
        }
    }
    
    Ok(stocks)
}

// 将股票数据保存为CSV文件
fn save_to_csv(stocks: &Vec<StockData>, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = std::io::BufWriter::new(file);
    
    // 写入BOM标记以支持Excel正确显示中文
    writer.write_all(&[0xEF, 0xBB, 0xBF])?;
    
    // 写入CSV头
    writeln!(writer, "股票代码,股票名称,当前价格,涨跌幅,成交量,相对成交量,市值,市盈率,每股收益,盈利增长,股息率,行业,评级")?;
    
    // 写入每支股票数据
    for stock in stocks {
        writeln!(
            writer,
            "{},{},{:.2},{},{},{:.2},{},{:.2},{},{},{},{},{}",
            stock.code,
            stock.name,
            stock.price,
            stock.change_percent,
            stock.volume,
            stock.reletive_volume,
            stock.market_cap,
            stock.pe_ratio,
            stock.eps,
            stock.earnings_growth,
            stock.dividend_yield,
            stock.sector,
            stock.rating
        )?;
    }
    
    Ok(())
}

// 将股票数据保存为JSON文件
fn save_to_json(stocks: &Vec<StockData>, filename: &str) -> std::io::Result<()> {
    // 手动构建JSON结构，确保中文正确编码
    let mut json_content = String::from("[\n");
    
    for (i, stock) in stocks.iter().enumerate() {
        json_content.push_str("  {\n");
        json_content.push_str(&format!("    \"code\": \"{}\",\n", stock.code));
        json_content.push_str(&format!("    \"name\": \"{}\",\n", stock.name));
        json_content.push_str(&format!("    \"price\": {:.2},\n", stock.price));
        json_content.push_str(&format!("    \"change_percent\": \"{}\",\n", stock.change_percent));
        json_content.push_str(&format!("    \"volume\": \"{}\",\n", stock.volume));
        json_content.push_str(&format!("    \"relative_volume\": {:.2},\n", stock.reletive_volume));
        json_content.push_str(&format!("    \"market_cap\": \"{}\",\n", stock.market_cap));
        json_content.push_str(&format!("    \"pe_ratio\": {:.2},\n", stock.pe_ratio));
        json_content.push_str(&format!("    \"eps\": \"{}\",\n", stock.eps));
        json_content.push_str(&format!("    \"earnings_growth\": \"{}\",\n", stock.earnings_growth));
        json_content.push_str(&format!("    \"dividend_yield\": \"{}\",\n", stock.dividend_yield));
        json_content.push_str(&format!("    \"sector\": \"{}\",\n", stock.sector));
        json_content.push_str(&format!("    \"rating\": \"{}\"\n", stock.rating));
        
        if i < stocks.len() - 1 {
            json_content.push_str("  },\n");
        } else {
            json_content.push_str("  }\n");
        }
    }
    
    json_content.push_str("]\n");
    
    // 写入文件，使用UTF-8编码
    let file = File::create(filename)?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(json_content.as_bytes())?;
    
    Ok(())
}

// 显示获取到的股票数据
fn display_stocks(stocks: &Vec<StockData>) {
    println!("\n总共获取到 {} 支股票数据：", stocks.len());
    for stock in stocks {
        stock.display();
    }
}

// 保存股票数据到文件
fn save_stock_data(stocks: &Vec<StockData>) {
    // 保存数据到文件
    match save_to_csv(&stocks, "stock_data.csv") {
        Ok(_) => println!("数据已保存到 stock_data.csv"),
        Err(e) => println!("保存CSV文件失败: {}", e),
    }
    
    match save_to_json(&stocks, "stock_data.json") {
        Ok(_) => println!("数据已保存到 stock_data.json"),
        Err(e) => println!("保存JSON文件失败: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caps = init_webdriver_config();
    let mut chrome_driver = start_chromedriver()?;
    let client = fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:9516")
        .await?;
    client.goto("https://cn.tradingview.com/screener/").await?;
    println!("等待页面加载中...");
    scroll_to_load_all(&client).await?;
    let stocks = fetch_stock_data_with_js(&client).await?;
    save_stock_data(&stocks);
    client.close().await?;
    chrome_driver.kill()?;
    Ok(())
}

