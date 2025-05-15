use crate::tabs::TabType;

/// 根据标签页类型获取对应的数据抓取脚本
pub fn get_data_extraction_script(tab: &TabType) -> String {
    format!(
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
                            // 处理技术评级, 这可能是带有div的
                            const technicalRatingDiv = cells[1].querySelector('div');
                            rowData.technicalRating = technicalRatingDiv ? technicalRatingDiv.textContent.trim() : cells[1].textContent.trim();
                        }}
                        if (cells[2]) {{
                            // 处理MA评级, 这可能是带有div的
                            const maRatingDiv = cells[2].querySelector('div');
                            rowData.maRating = maRatingDiv ? maRatingDiv.textContent.trim() : cells[2].textContent.trim();
                        }}
                        if (cells[3]) {{
                            // 处理振荡指标评级, 这可能是带有div的
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
        "#, tab.id()
    )
}

/// 获取检查标签页是否加载完成的脚本
pub fn get_tab_loaded_check_script() -> &'static str {
    r#"
    return document.querySelector('.overlay-gZJAyxim') === null;
    "#
}
/// 获取检查页面是否完全加载的脚本
pub fn get_page_loaded_check_script() -> &'static str {
    "return document.readyState === 'complete';"
}

/// 获取滚动加载脚本
pub fn get_scroll_script() -> &'static str {
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
            return true;
        }
    }
    return false;
    "#
}

/// 获取表格行数的脚本
pub fn get_row_count_script() -> &'static str {
    "return document.querySelectorAll('tbody[tabindex=\"100\"] tr').length;"
}

/// 获取标签页切换脚本
pub fn get_tab_click_script(tab_id: &str) -> String {
    format!(
        r#"
        var tab = document.querySelector('button[id="{}"]');
        if (tab) {{
            tab.click();
            return true;
        }}
        return false;
        "#,
        tab_id
    )
}

pub fn update_search_input_script(code: &str) -> String {
    format!(
        r#"
        function sleep(ms) {{
            return new Promise(resolve => setTimeout(resolve, ms));
        }}

        async function simulateUserTyping(element, text) {{
            element.focus();
            element.select();
            element.value = '';
            
            // 模拟逐个字符输入
            for(let i = 0; i < text.length; i++) {{
                element.value += text[i];
                await sleep(500);

                // 触发多种事件
                element.dispatchEvent(new InputEvent('input', {{bubbles: true, data: text[i], inputType: 'insertText'}}));
                element.dispatchEvent(new KeyboardEvent('keydown', {{key: text[i], code: `Key${{text[i].toUpperCase()}}`}}));
                element.dispatchEvent(new KeyboardEvent('keypress', {{key: text[i], code: `Key${{text[i].toUpperCase()}}`}}));
                element.dispatchEvent(new KeyboardEvent('keyup', {{key: text[i], code: `Key${{text[i].toUpperCase()}}`}}));
            }}
        }}

        var searchButton = document.querySelector('.searchButton-cfjBjL5J');
        if (searchButton) {{
            searchButton.click();
        }}

        const container = document.querySelector('span[data-qa-id="ui-lib-Input"]');
        const input = container.querySelector('input');

        const mouseEvents = ['mousedown', 'mouseup', 'click'];
        mouseEvents.forEach(type => {{
            container.dispatchEvent(new MouseEvent(type, {{
                bubbles: true,
                cancelable: true,
                view: window
            }}));
        }});

        input.focus();

        simulateUserTyping(input, '{}');
        
        "#,
        code
    )
    
}