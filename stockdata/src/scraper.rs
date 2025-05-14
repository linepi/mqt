use std::process::Command;
use fantoccini::{Client, ClientBuilder};
use serde_json::{self, Value};
use crate::tabs::TabType;
use crate::models::StockData;

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
        "useAutomationExtension": false
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

pub async fn scroll_to_load_all(client: &fantoccini::Client) -> Result<(), Box<dyn std::error::Error>> {
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

// 切换到指定标签页
pub async fn switch_to_tab(client: &Client, tab: TabType) -> Result<(), Box<dyn std::error::Error>> {
    println!("切换到{}标签页...", tab.name());
    
    let js_click = format!(
        r#"
        var tab = document.querySelector('button[id="{}"]');
        if (tab) {{
            tab.click();
            return true;
        }}
        return false;
        "#,
        tab.id()
    );
    
    let clicked = client.execute(&js_click, vec![]).await?;
    
    if !clicked.as_bool().unwrap_or(false) {
        return Err(format!("找不到{}标签页", tab.name()).into());
    }
    
    // 等待标签页加载完成
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    Ok(())
}

// 使用JavaScript执行数据抓取，获取所有标签页的股票数据
pub async fn fetch_stock_data(client: &Client) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    println!("开始从所有标签页获取股票数据...");
    
    // 存储每个标签页的数据集合
    let mut tab_data_sources = Vec::new();
    
    // 获取所有标签页的数据
    for tab in TabType::all().iter() {
        match fetch_stock_data_from_tab(client, *tab).await {
            Ok(tab_stocks) => {
                println!("成功获取{}标签页数据: {}支股票", tab.name(), tab_stocks.len());
                tab_data_sources.push(tab_stocks);
            },
            Err(e) => {
                eprintln!("获取{}标签页数据失败: {}", tab.name(), e);
            }
        }
    }
    
    // 使用merge_stock_data_sources函数合并所有标签页的数据
    let all_stock_data = crate::io::merge_stock_data_sources(&tab_data_sources);
    
    println!("成功获取并合并{}支股票的数据", all_stock_data.len());
    Ok(all_stock_data)
}

// 从指定标签页获取股票数据
pub async fn fetch_stock_data_from_tab(client: &Client, tab: TabType) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    // 切换到指定标签页
    switch_to_tab(client, tab).await?;
    
    // 执行JavaScript获取实际内容
    println!("正在从{}标签页获取数据...", tab.name());
    
    // 获取原始JavaScript执行结果
    let js_result = client.execute(&format!(
        r#"
        const rows = document.querySelectorAll('tbody[tabindex="100"] tr');
        const result = [];
        
        for (let i = 0; i < rows.length; i++) {{
            const cells = rows[i].querySelectorAll('td');
            const rowData = {{}};
            
            if (cells.length > 0) {{
                // 股票代码和名称 (始终在第一列)
                const codeCell = cells[0].querySelector('.tickerName-GrtoTeat');
                const nameCell = cells[0].querySelector('.tickerDescription-GrtoTeat');
                if (codeCell) rowData.code = codeCell.textContent.trim();
                if (nameCell) rowData.name = nameCell.textContent.trim();
                
                const tabId = "{}";
                
                // 根据不同标签页处理不同的数据
                switch(tabId) {{
                    case 'overview':
                        // 概览标签页
                        if (cells[1]) rowData.price = cells[1].textContent.trim().replace('CNY', '').replace(',', '');
                        if (cells[2]) {{
                            const changeEl = cells[2].querySelector('span');
                            rowData.changePercent = changeEl ? changeEl.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) rowData.volume = cells[3].textContent.trim();
                        if (cells[4]) rowData.relativeVolume = cells[4].textContent.trim();
                        if (cells[5]) rowData.marketCap = cells[5].textContent.trim();
                        if (cells[6]) rowData.peRatio = cells[6].textContent.trim();
                        if (cells[7]) rowData.eps = cells[7].textContent.trim();
                        if (cells[8]) {{
                            const growthEl = cells[8].querySelector('span');
                            rowData.earningsGrowth = growthEl ? growthEl.textContent.trim() : cells[8].textContent.trim();
                        }}
                        if (cells[9]) rowData.dividendYield = cells[9].textContent.trim();
                        if (cells[10]) {{
                            const sectorLink = cells[10].querySelector('a');
                            if (sectorLink) {{
                                rowData.sector = sectorLink.textContent.trim();
                            }} else {{
                                rowData.sector = cells[10].textContent.trim();
                            }}
                        }}
                        if (cells[11]) {{
                            const ratingDiv = cells[11].querySelector('div');
                            rowData.rating = ratingDiv ? ratingDiv.textContent.trim() : cells[11].textContent.trim();
                        }}
                        break;
                    case 'performance':
                        // 表现标签页
                        if (cells[1]) rowData.price = cells[1].textContent.trim().replace('CNY', '').replace(',', '');
                        if (cells[2]) {{
                            const changeEl = cells[2].querySelector('span');
                            rowData.changePercent = changeEl ? changeEl.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) {{
                            const perfEl = cells[3].querySelector('span');
                            rowData.performance1w = perfEl ? perfEl.textContent.trim() : cells[3].textContent.trim();
                        }}
                        if (cells[4]) {{
                            const perfEl = cells[4].querySelector('span');
                            rowData.performance1m = perfEl ? perfEl.textContent.trim() : cells[4].textContent.trim();
                        }}
                        if (cells[5]) {{
                            const perfEl = cells[5].querySelector('span');
                            rowData.performance3m = perfEl ? perfEl.textContent.trim() : cells[5].textContent.trim();
                        }}
                        if (cells[6]) {{
                            const perfEl = cells[6].querySelector('span');
                            rowData.performance6m = perfEl ? perfEl.textContent.trim() : cells[6].textContent.trim();
                        }}
                        if (cells[7]) {{
                            const perfEl = cells[7].querySelector('span');
                            rowData.performanceYtd = perfEl ? perfEl.textContent.trim() : cells[7].textContent.trim();
                        }}
                        if (cells[8]) {{
                            const perfEl = cells[8].querySelector('span');
                            rowData.performance1y = perfEl ? perfEl.textContent.trim() : cells[8].textContent.trim();
                        }}
                        if (cells[9]) {{
                            const perfEl = cells[9].querySelector('span');
                            rowData.performance5y = perfEl ? perfEl.textContent.trim() : cells[9].textContent.trim();
                        }}
                        if (cells[10]) {{
                            const perfEl = cells[10].querySelector('span');
                            rowData.performance10y = perfEl ? perfEl.textContent.trim() : cells[10].textContent.trim();
                        }}
                        if (cells[11]) {{
                            const perfEl = cells[11].querySelector('span');
                            rowData.performanceAll = perfEl ? perfEl.textContent.trim() : cells[11].textContent.trim();
                        }}
                        if (cells[12]) rowData.volatility1w = cells[12].textContent.trim();
                        if (cells[13]) rowData.volatility1m = cells[13].textContent.trim();
                        break;
                    case 'extendedHours':
                        // 延长时段标签页
                        if (cells[1]) rowData.preMarketClose = cells[1].textContent.trim().replace('CNY', '').replace(',', '');
                        if (cells[2]) {{
                            const changeEl = cells[2].querySelector('span');
                            rowData.preMarketChange = changeEl ? changeEl.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) {{
                            const gapEl = cells[3].querySelector('span');
                            rowData.preMarketGap = gapEl ? gapEl.textContent.trim() : cells[3].textContent.trim();
                        }}
                        if (cells[4]) rowData.preMarketVolume = cells[4].textContent.trim();
                        if (cells[5]) rowData.price = cells[5].textContent.trim().replace('CNY', '').replace(',', '');
                        if (cells[6]) {{
                            const changeEl = cells[6].querySelector('span');
                            rowData.changePercent = changeEl ? changeEl.textContent.trim() : cells[6].textContent.trim();
                        }}
                        if (cells[7]) {{
                            const gapEl = cells[7].querySelector('span');
                            rowData.gap = gapEl ? gapEl.textContent.trim() : cells[7].textContent.trim();
                        }}
                        if (cells[8]) rowData.volume = cells[8].textContent.trim();
                        if (cells[9]) {{
                            const volChangeEl = cells[9].querySelector('span');
                            rowData.volumeChange = volChangeEl ? volChangeEl.textContent.trim() : cells[9].textContent.trim();
                        }}
                        if (cells[10]) rowData.postMarketClose = cells[10].textContent.trim().replace('CNY', '').replace(',', '');
                        if (cells[11]) {{
                            const changeEl = cells[11].querySelector('span');
                            rowData.postMarketChange = changeEl ? changeEl.textContent.trim() : cells[11].textContent.trim();
                        }}
                        if (cells[12]) rowData.postMarketVolume = cells[12].textContent.trim();
                        break;
                    case 'valuation':
                        // 估值标签页
                        if (cells[1]) rowData.marketCap = cells[1].textContent.trim();
                        if (cells[2]) {{
                            const mcapPerfEl = cells[2].querySelector('span');
                            rowData.marketCapPerf1y = mcapPerfEl ? mcapPerfEl.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) rowData.peRatio = cells[3].textContent.trim();
                        if (cells[4]) rowData.pegRatio = cells[4].textContent.trim();
                        if (cells[5]) rowData.priceToSales = cells[5].textContent.trim();
                        if (cells[6]) rowData.priceToBook = cells[6].textContent.trim();
                        if (cells[7]) rowData.priceToCashFlow = cells[7].textContent.trim();
                        if (cells[8]) rowData.priceToFreeCashFlow = cells[8].textContent.trim();
                        if (cells[9]) rowData.priceToCash = cells[9].textContent.trim();
                        if (cells[10]) rowData.enterpriseValue = cells[10].textContent.trim();
                        if (cells[11]) rowData.evToRevenue = cells[11].textContent.trim();
                        if (cells[12]) rowData.evToEbit = cells[12].textContent.trim();
                        if (cells[13]) rowData.evToEbitda = cells[13].textContent.trim();
                        break;
                    case 'dividends':
                        // 股利标签页
                        if (cells[1]) rowData.dividendsPerShareYearly = cells[1].textContent.trim();
                        if (cells[2]) rowData.dividendsPerShareQuarterly = cells[2].textContent.trim();
                        if (cells[3]) rowData.dividendYield = cells[3].textContent.trim();
                        if (cells[4]) rowData.dividendYieldForward = cells[4].textContent.trim();
                        if (cells[5]) rowData.dividendPayoutRatio = cells[5].textContent.trim();
                        if (cells[6]) {{
                            const dpsGrowthEl = cells[6].querySelector('span');
                            rowData.dividendsPerShareGrowth = dpsGrowthEl ? dpsGrowthEl.textContent.trim() : cells[6].textContent.trim();
                        }}
                        if (cells[7]) rowData.continuousDividendPayout = cells[7].textContent.trim();
                        if (cells[8]) rowData.continuousDividendGrowth = cells[8].textContent.trim();
                        break;
                    case 'profitability':
                        // 盈利能力标签页
                        if (cells[1]) rowData.grossMargin = cells[1].textContent.trim();
                        if (cells[2]) rowData.operatingMargin = cells[2].textContent.trim();
                        if (cells[3]) rowData.profitMargin = cells[3].textContent.trim();
                        if (cells[4]) rowData.pureMargin = cells[4].textContent.trim();
                        if (cells[5]) rowData.freeCashFlowMargin = cells[5].textContent.trim();
                        if (cells[6]) rowData.roi = cells[6].textContent.trim();
                        if (cells[7]) rowData.roe = cells[7].textContent.trim();
                        if (cells[8]) rowData.roic = cells[8].textContent.trim();
                        if (cells[9]) rowData.rdRatio = cells[9].textContent.trim();
                        if (cells[10]) rowData.sgaRatio = cells[10].textContent.trim();
                        break;
                    case 'incomeStatement':
                        // 损益表标签页
                        if (cells[1]) rowData.totalRevenue = cells[1].textContent.trim();
                        if (cells[2]) {{
                            const growthEl = cells[2].querySelector('span');
                            rowData.revenueGrowth = growthEl ? growthEl.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) rowData.grossProfit = cells[3].textContent.trim();
                        if (cells[4]) rowData.operatingIncome = cells[4].textContent.trim();
                        if (cells[5]) rowData.netIncome = cells[5].textContent.trim();
                        if (cells[6]) rowData.ebitda = cells[6].textContent.trim();
                        if (cells[7]) rowData.epsDiluted = cells[7].textContent.trim();
                        if (cells[8]) {{
                            const growthEl = cells[8].querySelector('span');
                            rowData.epsDilutedGrowth = growthEl ? growthEl.textContent.trim() : cells[8].textContent.trim();
                        }}
                        break;
                    case 'balanceSheet':
                        // 资产负债表标签页
                        if (cells[1]) rowData.totalAssets = cells[1].textContent.trim();
                        if (cells[2]) rowData.totalCurrentAssets = cells[2].textContent.trim();
                        if (cells[3]) rowData.cashAndShortTerm = cells[3].textContent.trim();
                        if (cells[4]) rowData.totalLiabilities = cells[4].textContent.trim();
                        if (cells[5]) rowData.totalDebt = cells[5].textContent.trim();
                        if (cells[6]) rowData.netDebt = cells[6].textContent.trim();
                        if (cells[7]) rowData.totalEquity = cells[7].textContent.trim();
                        if (cells[8]) rowData.currentRatio = cells[8].textContent.trim();
                        if (cells[9]) rowData.quickRatio = cells[9].textContent.trim();
                        if (cells[10]) rowData.debtToEquity = cells[10].textContent.trim();
                        if (cells[11]) rowData.cashToDebt = cells[11].textContent.trim();
                        break;
                    case 'cashFlow':
                        // 现金流标签页
                        if (cells[1]) rowData.operatingCashFlow = cells[1].textContent.trim();
                        if (cells[2]) rowData.investingCashFlow = cells[2].textContent.trim();
                        if (cells[3]) rowData.financingCashFlow = cells[3].textContent.trim();
                        if (cells[4]) rowData.freeCashFlow = cells[4].textContent.trim();
                        if (cells[5]) rowData.capitalExpenditures = cells[5].textContent.trim();
                        break;
                    case 'technicals':
                        // 技术指标标签页
                        if (cells[1]) {{
                            // 处理技术评级，这可能是带有div的
                            const technicalRatingDiv = cells[1].querySelector('div');
                            rowData.technicalRating = technicalRatingDiv ? technicalRatingDiv.textContent.trim() : cells[1].textContent.trim();
                        }}
                        if (cells[2]) {{
                            // 处理MA评级，这可能是带有div的
                            const maRatingDiv = cells[2].querySelector('div');
                            rowData.maRating = maRatingDiv ? maRatingDiv.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) {{
                            // 处理振荡指标评级，这可能是带有div的
                            const oscRatingDiv = cells[3].querySelector('div');
                            rowData.oscillatorsRating = oscRatingDiv ? oscRatingDiv.textContent.trim() : cells[3].textContent.trim();
                        }}
                        if (cells[4]) rowData.rsi14 = cells[4].textContent.trim();
                        if (cells[5]) rowData.momentum10 = cells[5].textContent.trim();
                        if (cells[6]) rowData.awesomeOscillator = cells[6].textContent.trim();
                        if (cells[7]) rowData.cci20 = cells[7].textContent.trim();
                        if (cells[8]) rowData.stochasticK = cells[8].textContent.trim();
                        if (cells[9]) rowData.stochasticD = cells[9].textContent.trim();
                        if (cells[10]) rowData.candlestickPattern = cells[10].textContent.trim();
                        break;
                    default:
                        console.log("Unknown tab: " + tabId);
                }}
            }}
            
            if (Object.keys(rowData).length > 0) {{
                result.push(rowData);
            }}
        }}
        
        return JSON.stringify(result);
        "#, tab.id()),
        vec![],
    ).await?;
    
    // 将JavaScript结果转换为JSON值
    let json_str = js_result.as_str().unwrap_or("[]");
    let json_data: Value = serde_json::from_str(json_str)?;
    
    // 使用解析器将JSON值转换为StockData对象
    let stocks = crate::parser::parse_stock_data_from_json(json_data, tab)?;
    
    println!("已从{}标签页获取{}支股票的数据", tab.name(), stocks.len());
    Ok(stocks)
} 