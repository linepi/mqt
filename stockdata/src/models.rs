use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Rating {
    StrongBuy,   // 强烈买入
    Buy,         // 买入
    Neutral,     // 中性
    Sell,        // 卖出
    StrongSell,  // 强烈卖出
    Unknown      // 未知评级
}

impl Rating {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "强烈买入" => Rating::StrongBuy,
            "买入" => Rating::Buy,
            "中立" => Rating::Neutral,
            "卖出" => Rating::Sell,
            "强烈卖出" => Rating::StrongSell,
            _ => Rating::Unknown
        }
    }
}

impl Default for Rating {
    fn default() -> Self {
        Rating::Unknown
    }
}

// 定义股票数据结构
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StockData {
    // 基本信息
    pub code: String,                // 股票代码
    pub name: String,                // 股票名称
    pub price: f64,                  // 当前价格
    
    // 概览标签页
    pub change_percent: f64,      // 涨跌幅, 负值表示下跌, 正值表示上涨
    pub volume: i64,              // 成交量
    pub relative_volume: f64,        // 相对成交量
    pub market_cap: i64,          // 市值（单位：人民币）
    pub pe_ratio: f64,               // 市盈率(P/E)
    pub eps: f64,                 // 每股收益(单位：人民币)
    pub earnings_growth: f64,     // 盈利增长
    pub dividend_yield: f64,      // 股息率
    pub sector: String,              // 行业
    pub rating: Rating,              // 分析师评级
    
    // 表现标签页
    pub performance_1w: f64,      // 表现 1周
    pub performance_1m: f64,      // 表现 1个月
    pub performance_3m: f64,      // 表现 3个月
    pub performance_6m: f64,      // 表现 6个月
    pub performance_ytd: f64,     // 表现 今年至今
    pub performance_1y: f64,      // 表现 1年
    pub performance_5y: f64,      // 表现 5年
    pub performance_10y: f64,     // 表现 10年
    pub performance_all: f64,     // 表现 全部时间
    pub volatility_1w: f64,          // 波动率 1周
    pub volatility_1m: f64,          // 波动率 1个月
    
    // 延长时段标签页
    pub pre_market_close: f64,       // 盘前结束价格
    pub pre_market_change: f64,   // 盘前涨跌幅
    pub pre_market_gap: f64,      // 盘前跳空
    pub pre_market_volume: i64,   // 盘前成交量
                              // 价格(占位)
                              // 更改(占位)
    pub gap: f64,                 // 日内跳空
                              // 成交量(占位)
    pub volume_change: f64,       // 成交量变动
    pub post_market_close: f64,      // 盘后时段价格
    pub post_market_change: f64,  // 盘后涨跌幅
    pub post_market_volume: i64,  // 盘后成交量
    
    // 估值标签页
    pub market_cap_perf_1y: f64,  // 市值表现 1年
    pub peg_ratio: f64,              // 市盈率增长比(PEG)
    pub price_to_sales: f64,         // 市销率(P/S)
    pub price_to_book: f64,          // 市净率(P/B)
    pub price_to_cash_flow: f64,     // 价格现金流比率(P/CF)
    pub price_to_free_cash_flow: f64,// 价格自由现金流比率(P/FCF)
    pub price_to_cash: f64,          // 价格现金比
    pub enterprise_value: i64,    // 企业价值(EV)
    pub ev_to_revenue: f64,          // 企业价值收入比(EV/Revenue)
    pub ev_to_ebit: f64,             // 企业价值息税前利润比(EV/EBIT)
    pub ev_to_ebitda: f64,           // 企业价值息税折旧摊销前利润比(EV/EBITDA)
    
    // 股利标签页
    pub dividends_per_share_yearly: f64,  // 每股股息(年度)
    pub dividends_per_share_quarterly: f64,// 每股股息(季度)
    pub dividend_payout_ratio: f64,       // 派息率
    pub dividends_per_share_growth: f64,  // 每股股息增长
    pub continuous_dividend_payout: i64,  // 持续派息
    pub continuous_dividend_growth: i64,  // 股息持续增长
    
    // 盈利能力标签页
    pub gross_margin: f64,                // 毛利率
    pub operating_margin: f64,            // 经营利润率
    pub profit_margin: f64,               // 税前利润率
    pub pure_margin: f64,                 // 净利率
    pub free_cash_flow_margin: f64,       // 自由现金流保证金
    pub roi: f64,                         // 资产收益率
    pub roe: f64,                         // 净资产收益率
    pub roic: f64,                        // 投资资本回报率
    pub rd_ratio: f64,                    // 研发比率
    pub sga_ratio: f64,                   // 销售及管理费用比率
    
    // 损益表标签页
    pub total_revenue: i64,               // 总收入(TTM)
    pub revenue_growth: f64,              // 收入增长(TTM同比)
    pub gross_profit: i64,                // 毛利润(TTM)
    pub operating_income: i64,            // 营业收入(TTM)
    pub net_income: i64,                  // 净收入(TTM)
    pub ebitda: i64,                      // 税息折旧及摊销前利润(TTM)
    pub eps_diluted: f64,                 // 摊薄每股收益(TTM) 
    pub eps_diluted_growth: f64,          // 每股收益稀释增长(TTM同比)
    
    // 资产负债表标签页
    pub total_assets: i64,                // 总资产
    pub total_current_assets: i64,        // 流动资产
    pub cash_and_short_term: i64,         // 手头现金
    pub total_liabilities: i64,           // 总负债
    pub total_debt: i64,                  // 总债务
    pub net_debt: i64,                    // 净债务
    pub total_equity: i64,                // 权益总额
    pub current_ratio: f64,                  // 流动比率
    pub quick_ratio: f64,                    // 速动比率
    pub debt_to_equity: f64,                 // 债务权益比
    pub cash_to_debt: f64,                   // 现金债务比率
    
    // 现金流标签页
    pub operating_cash_flow: i64,         // 经营CF
    pub investing_cash_flow: i64,         // 投资CF
    pub financing_cash_flow: i64,         // 融资CF
    pub free_cash_flow: i64,              // 自由现金流
    pub capital_expenditures: i64,        // 资本支出
    
    // 技术指标标签页
    pub technical_rating: Rating,            // 技术评级
    pub ma_rating: Rating,                   // MA评级
    pub oscillators_rating: Rating,          // 振荡指标评级
    pub rsi_14: f64,                         // RSI(14)
    pub momentum_10: f64,                    // 动量(10)
    pub awesome_oscillator: f64,             // AO动量震荡指标
    pub cci_20: f64,                         // 顺势指标(20)
    pub stochastic_k: f64,                   // 随机指数K
    pub stochastic_d: f64,                   // 随机指数D
    pub candlestick_pattern: String,         // K线形态
}

// 字符串转换为数值的辅助函数
pub fn parse_f64(s: &str) -> f64 {
    // 移除百分号、逗号、货币符号和引号, 并转换为浮点数
    let s = s.trim()
        .replace("%", "")
        .replace(',', "")
        .replace("CNY", "")
        .replace('"', "")
        .replace(' ', "");
    
    if s.is_empty() || s == "—" || s == "-" || s == "−" {
        return -404.0;
    }
    
    // 处理带有正负号的值
    let multiplier = if s.starts_with('+') { 1.0 } 
                     else if s.starts_with('-') { -1.0 } 
                     else if s.starts_with('—') { -1.0 } 
                     else if s.starts_with('−') { -1.0 } 
                     else { 1.0 };
    
    // 提取数字部分
    let num_str = s.trim_start_matches('+').trim_start_matches('-').trim_start_matches('—').trim_start_matches('−');
    
    match num_str.parse::<f64>() {
        Ok(val) => val * multiplier,
        Err(_) => -500.0
    }
}

// 解析百分比为浮点数
pub fn parse_percentage(s: &str) -> f64 {
    let value = parse_f64(s);
    // 百分比返回的是实际数值, 不需要除以100
    value
}

// 解析大数值（带B,M,K后缀的）到i64
pub fn parse_large_number(s: &str) -> i64 {
    let s = s.trim()
        .replace(',', "")
        .replace("CNY", "")
        .replace('"', "")
        .replace(' ', "");
    
    if s.is_empty() || s == "—" || s == "-" || s == "−" {
        return -404;
    }

    let multiplier = if s.starts_with('+') { 1.0 } 
                     else if s.starts_with('-') { -1.0 } 
                     else if s.starts_with('—') { -1.0 } 
                     else if s.starts_with('−') { -1.0 } 
                     else { 1.0 };
    
    // 处理带有单位的值
    let mut value = 0.0;
    let s = s.trim_start_matches('+').trim_start_matches('-').trim_start_matches('—').trim_start_matches('−');
    
    if s.contains('T') {
        // 兆（万亿）
        if let Some(num_str) = s.split('T').next() {
            if let Ok(num) = num_str.trim().parse::<f64>() {
                value = num * 1_000_000_000_000.0 * multiplier;
            }
        }
    } else if s.contains('B') {
        // 十亿
        if let Some(num_str) = s.split('B').next() {
            if let Ok(num) = num_str.trim().parse::<f64>() {
                value = num * 1_000_000_000.0 * multiplier;
            }
        }
    } else if s.contains('M') {
        // 百万
        if let Some(num_str) = s.split('M').next() {
            if let Ok(num) = num_str.trim().parse::<f64>() {
                value = num * 1_000_000.0 * multiplier;
            }
        }
    } else if s.contains('K') {
        // 千
        if let Some(num_str) = s.split('K').next() {
            if let Ok(num) = num_str.trim().parse::<f64>() {
                value = num * 1_000.0 * multiplier;
            }
        }
    } else {
        // 没有单位, 直接解析
        if let Ok(num) = s.parse::<f64>() {
            value = num * multiplier;
        }
    }
    
    value as i64
}

// 合并两个股票数据, 将src中的非零值合并到dest中
pub fn merge_stock_data(dest: &mut StockData, src: &StockData) {
    // 基本信息
    if dest.name.is_empty() && !src.name.is_empty() {
        dest.name = src.name.clone();
    }
    
    if dest.price == 0.0 && src.price != 0.0 {
        dest.price = src.price;
    }
    
    if dest.change_percent == 0.0 && src.change_percent != 0.0 {
        dest.change_percent = src.change_percent;
    }
    
    // 概览标签页数据
    if dest.volume == 0 && src.volume != 0 {
        dest.volume = src.volume;
    }
    
    if dest.relative_volume == 0.0 && src.relative_volume != 0.0 {
        dest.relative_volume = src.relative_volume;
    }
    
    if dest.market_cap == 0 && src.market_cap != 0 {
        dest.market_cap = src.market_cap;
    }
    
    if dest.pe_ratio == 0.0 && src.pe_ratio != 0.0 {
        dest.pe_ratio = src.pe_ratio;
    }
    
    if dest.eps == 0.0 && src.eps != 0.0 {
        dest.eps = src.eps;
    }
    
    if dest.earnings_growth == 0.0 && src.earnings_growth != 0.0 {
        dest.earnings_growth = src.earnings_growth;
    }
    
    if dest.dividend_yield == 0.0 && src.dividend_yield != 0.0 {
        dest.dividend_yield = src.dividend_yield;
    }
    
    if dest.sector.is_empty() && !src.sector.is_empty() {
        dest.sector = src.sector.clone();
    }
    
    if dest.rating == Rating::Unknown && src.rating != Rating::Unknown {
        dest.rating = src.rating.clone();
    }
    
    // 表现标签页数据
    if dest.performance_1w == 0.0 && src.performance_1w != 0.0 {
        dest.performance_1w = src.performance_1w;
    }
    
    if dest.performance_1m == 0.0 && src.performance_1m != 0.0 {
        dest.performance_1m = src.performance_1m;
    }
    
    if dest.performance_3m == 0.0 && src.performance_3m != 0.0 {
        dest.performance_3m = src.performance_3m;
    }
    
    if dest.performance_6m == 0.0 && src.performance_6m != 0.0 {
        dest.performance_6m = src.performance_6m;
    }
    
    if dest.performance_ytd == 0.0 && src.performance_ytd != 0.0 {
        dest.performance_ytd = src.performance_ytd;
    }
    
    if dest.performance_1y == 0.0 && src.performance_1y != 0.0 {
        dest.performance_1y = src.performance_1y;
    }
    
    if dest.performance_5y == 0.0 && src.performance_5y != 0.0 {
        dest.performance_5y = src.performance_5y;
    }
    
    if dest.performance_10y == 0.0 && src.performance_10y != 0.0 {
        dest.performance_10y = src.performance_10y;
    }
    
    if dest.performance_all == 0.0 && src.performance_all != 0.0 {
        dest.performance_all = src.performance_all;
    }
    
    if dest.volatility_1w == 0.0 && src.volatility_1w != 0.0 {
        dest.volatility_1w = src.volatility_1w;
    }
    
    if dest.volatility_1m == 0.0 && src.volatility_1m != 0.0 {
        dest.volatility_1m = src.volatility_1m;
    }
    
    // 延长时段标签页数据
    if dest.pre_market_close == 0.0 && src.pre_market_close != 0.0 {
        dest.pre_market_close = src.pre_market_close;
    }
    
    if dest.pre_market_change == 0.0 && src.pre_market_change != 0.0 {
        dest.pre_market_change = src.pre_market_change;
    }
    
    if dest.pre_market_gap == 0.0 && src.pre_market_gap != 0.0 {
        dest.pre_market_gap = src.pre_market_gap;
    }
    
    if dest.pre_market_volume == 0 && src.pre_market_volume != 0 {
        dest.pre_market_volume = src.pre_market_volume;
    }
    
    if dest.gap == 0.0 && src.gap != 0.0 {
        dest.gap = src.gap;
    }
    
    if dest.volume_change == 0.0 && src.volume_change != 0.0 {
        dest.volume_change = src.volume_change;
    }
    
    if dest.post_market_close == 0.0 && src.post_market_close != 0.0 {
        dest.post_market_close = src.post_market_close;
    }
    
    if dest.post_market_change == 0.0 && src.post_market_change != 0.0 {
        dest.post_market_change = src.post_market_change;
    }
    
    if dest.post_market_volume == 0 && src.post_market_volume != 0 {
        dest.post_market_volume = src.post_market_volume;
    }
    
    // 估值标签页数据
    if dest.market_cap_perf_1y == 0.0 && src.market_cap_perf_1y != 0.0 {
        dest.market_cap_perf_1y = src.market_cap_perf_1y;
    }
    
    if dest.peg_ratio == 0.0 && src.peg_ratio != 0.0 {
        dest.peg_ratio = src.peg_ratio;
    }
    
    if dest.price_to_sales == 0.0 && src.price_to_sales != 0.0 {
        dest.price_to_sales = src.price_to_sales;
    }
    
    if dest.price_to_book == 0.0 && src.price_to_book != 0.0 {
        dest.price_to_book = src.price_to_book;
    }
    
    if dest.price_to_cash_flow == 0.0 && src.price_to_cash_flow != 0.0 {
        dest.price_to_cash_flow = src.price_to_cash_flow;
    }
    
    if dest.price_to_free_cash_flow == 0.0 && src.price_to_free_cash_flow != 0.0 {
        dest.price_to_free_cash_flow = src.price_to_free_cash_flow;
    }
    
    if dest.price_to_cash == 0.0 && src.price_to_cash != 0.0 {
        dest.price_to_cash = src.price_to_cash;
    }
    
    if dest.enterprise_value == 0 && src.enterprise_value != 0 {
        dest.enterprise_value = src.enterprise_value;
    }
    
    if dest.ev_to_revenue == 0.0 && src.ev_to_revenue != 0.0 {
        dest.ev_to_revenue = src.ev_to_revenue;
    }
    
    if dest.ev_to_ebit == 0.0 && src.ev_to_ebit != 0.0 {
        dest.ev_to_ebit = src.ev_to_ebit;
    }
    
    if dest.ev_to_ebitda == 0.0 && src.ev_to_ebitda != 0.0 {
        dest.ev_to_ebitda = src.ev_to_ebitda;
    }
    
    // 股利标签页数据
    if dest.dividends_per_share_yearly == 0.0 && src.dividends_per_share_yearly != 0.0 {
        dest.dividends_per_share_yearly = src.dividends_per_share_yearly;
    }
    
    if dest.dividends_per_share_quarterly == 0.0 && src.dividends_per_share_quarterly != 0.0 {
        dest.dividends_per_share_quarterly = src.dividends_per_share_quarterly;
    }
    
    if dest.dividend_payout_ratio == 0.0 && src.dividend_payout_ratio != 0.0 {
        dest.dividend_payout_ratio = src.dividend_payout_ratio;
    }
    
    if dest.dividends_per_share_growth == 0.0 && src.dividends_per_share_growth != 0.0 {
        dest.dividends_per_share_growth = src.dividends_per_share_growth;
    }
    
    if dest.continuous_dividend_payout == 0 && src.continuous_dividend_payout != 0 {
        dest.continuous_dividend_payout = src.continuous_dividend_payout;
    }
    
    if dest.continuous_dividend_growth == 0 && src.continuous_dividend_growth != 0 {
        dest.continuous_dividend_growth = src.continuous_dividend_growth;
    }
    
    // 盈利能力标签页数据
    if dest.gross_margin == 0.0 && src.gross_margin != 0.0 {
        dest.gross_margin = src.gross_margin;
    }
    
    if dest.operating_margin == 0.0 && src.operating_margin != 0.0 {
        dest.operating_margin = src.operating_margin;
    }
    
    if dest.profit_margin == 0.0 && src.profit_margin != 0.0 {
        dest.profit_margin = src.profit_margin;
    }
    
    if dest.pure_margin == 0.0 && src.pure_margin != 0.0 {
        dest.pure_margin = src.pure_margin;
    }
    
    if dest.free_cash_flow_margin == 0.0 && src.free_cash_flow_margin != 0.0 {
        dest.free_cash_flow_margin = src.free_cash_flow_margin;
    }
    
    if dest.roi == 0.0 && src.roi != 0.0 {
        dest.roi = src.roi;
    }
    
    if dest.roe == 0.0 && src.roe != 0.0 {
        dest.roe = src.roe;
    }
    
    if dest.roic == 0.0 && src.roic != 0.0 {
        dest.roic = src.roic;
    }
    
    if dest.rd_ratio == 0.0 && src.rd_ratio != 0.0 {
        dest.rd_ratio = src.rd_ratio;
    }
    
    if dest.sga_ratio == 0.0 && src.sga_ratio != 0.0 {
        dest.sga_ratio = src.sga_ratio;
    }
    
    // 损益表标签页数据
    if dest.total_revenue == 0 && src.total_revenue != 0 {
        dest.total_revenue = src.total_revenue;
    }
    
    if dest.revenue_growth == 0.0 && src.revenue_growth != 0.0 {
        dest.revenue_growth = src.revenue_growth;
    }
    
    if dest.gross_profit == 0 && src.gross_profit != 0 {
        dest.gross_profit = src.gross_profit;
    }
    
    if dest.operating_income == 0 && src.operating_income != 0 {
        dest.operating_income = src.operating_income;
    }
    
    if dest.net_income == 0 && src.net_income != 0 {
        dest.net_income = src.net_income;
    }
    
    if dest.ebitda == 0 && src.ebitda != 0 {
        dest.ebitda = src.ebitda;
    }
    
    if dest.eps_diluted == 0.0 && src.eps_diluted != 0.0 {
        dest.eps_diluted = src.eps_diluted;
    }
    
    if dest.eps_diluted_growth == 0.0 && src.eps_diluted_growth != 0.0 {
        dest.eps_diluted_growth = src.eps_diluted_growth;
    }
    
    // 资产负债表标签页数据
    if dest.total_assets == 0 && src.total_assets != 0 {
        dest.total_assets = src.total_assets;
    }
    
    if dest.total_current_assets == 0 && src.total_current_assets != 0 {
        dest.total_current_assets = src.total_current_assets;
    }
    
    if dest.cash_and_short_term == 0 && src.cash_and_short_term != 0 {
        dest.cash_and_short_term = src.cash_and_short_term;
    }
    
    if dest.total_liabilities == 0 && src.total_liabilities != 0 {
        dest.total_liabilities = src.total_liabilities;
    }
    
    if dest.total_debt == 0 && src.total_debt != 0 {
        dest.total_debt = src.total_debt;
    }
    
    if dest.net_debt == 0 && src.net_debt != 0 {
        dest.net_debt = src.net_debt;
    }
    
    if dest.total_equity == 0 && src.total_equity != 0 {
        dest.total_equity = src.total_equity;
    }
    
    if dest.current_ratio == 0.0 && src.current_ratio != 0.0 {
        dest.current_ratio = src.current_ratio;
    }
    
    if dest.quick_ratio == 0.0 && src.quick_ratio != 0.0 {
        dest.quick_ratio = src.quick_ratio;
    }
    
    if dest.debt_to_equity == 0.0 && src.debt_to_equity != 0.0 {
        dest.debt_to_equity = src.debt_to_equity;
    }
    
    if dest.cash_to_debt == 0.0 && src.cash_to_debt != 0.0 {
        dest.cash_to_debt = src.cash_to_debt;
    }
    
    // 现金流标签页数据
    if dest.operating_cash_flow == 0 && src.operating_cash_flow != 0 {
        dest.operating_cash_flow = src.operating_cash_flow;
    }
    
    if dest.investing_cash_flow == 0 && src.investing_cash_flow != 0 {
        dest.investing_cash_flow = src.investing_cash_flow;
    }
    
    if dest.financing_cash_flow == 0 && src.financing_cash_flow != 0 {
        dest.financing_cash_flow = src.financing_cash_flow;
    }
    
    if dest.free_cash_flow == 0 && src.free_cash_flow != 0 {
        dest.free_cash_flow = src.free_cash_flow;
    }
    
    if dest.capital_expenditures == 0 && src.capital_expenditures != 0 {
        dest.capital_expenditures = src.capital_expenditures;
    }
    
    // 技术指标标签页数据
    if dest.technical_rating == Rating::Unknown && src.technical_rating != Rating::Unknown {
        dest.technical_rating = src.technical_rating.clone();
    }
    
    if dest.ma_rating == Rating::Unknown && src.ma_rating != Rating::Unknown {
        dest.ma_rating = src.ma_rating.clone();
    }
    
    if dest.oscillators_rating == Rating::Unknown && src.oscillators_rating != Rating::Unknown {
        dest.oscillators_rating = src.oscillators_rating.clone();
    }
    
    if dest.rsi_14 == 0.0 && src.rsi_14 != 0.0 {
        dest.rsi_14 = src.rsi_14;
    }
    
    if dest.momentum_10 == 0.0 && src.momentum_10 != 0.0 {
        dest.momentum_10 = src.momentum_10;
    }
    
    if dest.awesome_oscillator == 0.0 && src.awesome_oscillator != 0.0 {
        dest.awesome_oscillator = src.awesome_oscillator;
    }
    
    if dest.cci_20 == 0.0 && src.cci_20 != 0.0 {
        dest.cci_20 = src.cci_20;
    }
    
    if dest.stochastic_k == 0.0 && src.stochastic_k != 0.0 {
        dest.stochastic_k = src.stochastic_k;
    }
    
    if dest.stochastic_d == 0.0 && src.stochastic_d != 0.0 {
        dest.stochastic_d = src.stochastic_d;
    }
    
    if dest.candlestick_pattern.is_empty() && !src.candlestick_pattern.is_empty() {
        dest.candlestick_pattern = src.candlestick_pattern.clone();
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_f64() {
        assert_eq!(parse_f64("123.45%"), 123.45);
        assert_eq!(parse_f64("+123.45%"), 123.45);
        assert_eq!(parse_f64("-123.45%"), -123.45);
        assert_eq!(parse_f64("—123.45%"), -123.45);
        assert_eq!(parse_f64("−2.02%"), -123.45);
        assert_eq!(parse_f64("4.16 CNY"), 4.16);
        assert_eq!(parse_f64("-4.16 CNY"), -4.16);
    }
}
