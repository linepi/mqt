use serde_json::Value;
use crate::models::{StockData, Rating, parse_f64, parse_percentage, parse_large_number};
use crate::tabs::TabType;

// 从JSON解析股票数据
pub fn parse_stock_data_from_json(js_data: Value, tab: TabType) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    let mut stocks = Vec::new();
    
    if let Value::Array(items) = js_data {
        for item in items {
            let mut stock = StockData::default();
            
            // 提取股票代码和名称（每个标签页都有）
            if let Some(code) = item.get("code").and_then(|v| v.as_str()) {
                stock.code = code.to_string();
            } else {
                // 如果没有代码, 跳过此条记录
                continue;
            }
            
            if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                stock.name = name.to_string();
            }
            
            // 提取价格（通常在多个标签页中都有）
            if let Some(price_str) = item.get("price").and_then(|v| v.as_str()) {
                stock.price = parse_f64(price_str);
            }
            
            // 根据不同标签页处理不同的数据
            match tab {
                TabType::Overview => {
                    // 概览标签页数据
                    if let Some(change_pct) = item.get("changePercent").and_then(|v| v.as_str()) {
                        stock.change_percent = parse_percentage(change_pct);
                    }
                    
                    if let Some(volume) = item.get("volume").and_then(|v| v.as_str()) {
                        stock.volume = parse_large_number(volume);
                    }
                    
                    if let Some(rel_vol) = item.get("relativeVolume").and_then(|v| v.as_str()) {
                        stock.relative_volume = parse_f64(rel_vol);
                    }
                    
                    if let Some(mkt_cap) = item.get("marketCap").and_then(|v| v.as_str()) {
                        stock.market_cap = parse_large_number(mkt_cap);
                    }
                    
                    if let Some(pe) = item.get("peRatio").and_then(|v| v.as_str()) {
                        stock.pe_ratio = parse_f64(pe);
                    }
                    
                    if let Some(eps) = item.get("eps").and_then(|v| v.as_str()) {
                        stock.eps = parse_f64(eps);
                    }
                    
                    if let Some(earnings_g) = item.get("earningsGrowth").and_then(|v| v.as_str()) {
                        stock.earnings_growth = parse_percentage(earnings_g);
                    }
                    
                    if let Some(div_yield) = item.get("dividendYield").and_then(|v| v.as_str()) {
                        stock.dividend_yield = parse_percentage(div_yield);
                    }
                    
                    if let Some(sector) = item.get("sector").and_then(|v| v.as_str()) {
                        stock.sector = sector.to_string();
                    }
                    
                    if let Some(rating) = item.get("rating").and_then(|v| v.as_str()) {
                        stock.rating = Rating::from_str(rating);
                    }
                },
                TabType::Performance => {
                    // 表现标签页数据
                    if let Some(change_pct) = item.get("changePercent").and_then(|v| v.as_str()) {
                        stock.change_percent = parse_percentage(change_pct);
                    }
                    
                    if let Some(perf1w) = item.get("performance1w").and_then(|v| v.as_str()) {
                        stock.performance_1w = parse_percentage(perf1w);
                    }
                    
                    if let Some(perf1m) = item.get("performance1m").and_then(|v| v.as_str()) {
                        stock.performance_1m = parse_percentage(perf1m);
                    }
                    
                    if let Some(perf3m) = item.get("performance3m").and_then(|v| v.as_str()) {
                        stock.performance_3m = parse_percentage(perf3m);
                    }
                    
                    if let Some(perf6m) = item.get("performance6m").and_then(|v| v.as_str()) {
                        stock.performance_6m = parse_percentage(perf6m);
                    }
                    
                    if let Some(perfytd) = item.get("performanceYtd").and_then(|v| v.as_str()) {
                        stock.performance_ytd = parse_percentage(perfytd);
                    }
                    
                    if let Some(perf1y) = item.get("performance1y").and_then(|v| v.as_str()) {
                        stock.performance_1y = parse_percentage(perf1y);
                    }
                    
                    if let Some(perf5y) = item.get("performance5y").and_then(|v| v.as_str()) {
                        stock.performance_5y = parse_percentage(perf5y);
                    }
                    
                    if let Some(perf10y) = item.get("performance10y").and_then(|v| v.as_str()) {
                        stock.performance_10y = parse_percentage(perf10y);
                    }
                    
                    if let Some(perf_all) = item.get("performanceAll").and_then(|v| v.as_str()) {
                        stock.performance_all = parse_percentage(perf_all);
                    }
                    
                    if let Some(vol1w) = item.get("volatility1w").and_then(|v| v.as_str()) {
                        stock.volatility_1w = parse_f64(vol1w);
                    }
                    
                    if let Some(vol1m) = item.get("volatility1m").and_then(|v| v.as_str()) {
                        stock.volatility_1m = parse_f64(vol1m);
                    }
                },
                TabType::ExtendedHours => {
                    // 延长时段标签页数据
                    if let Some(pre_close) = item.get("preMarketClose").and_then(|v| v.as_str()) {
                        stock.pre_market_close = parse_f64(pre_close);
                    }
                    
                    if let Some(pre_change) = item.get("preMarketChange").and_then(|v| v.as_str()) {
                        stock.pre_market_change = parse_percentage(pre_change);
                    }
                    
                    if let Some(pre_gap) = item.get("preMarketGap").and_then(|v| v.as_str()) {
                        stock.pre_market_gap = parse_percentage(pre_gap);
                    }
                    
                    if let Some(pre_volume) = item.get("preMarketVolume").and_then(|v| v.as_str()) {
                        stock.pre_market_volume = parse_large_number(pre_volume);
                    }
                    
                    if let Some(gap) = item.get("gap").and_then(|v| v.as_str()) {
                        stock.gap = parse_percentage(gap);
                    }
                    
                    if let Some(vol_change) = item.get("volumeChange").and_then(|v| v.as_str()) {
                        stock.volume_change = parse_percentage(vol_change);
                    }
                    
                    if let Some(post_close) = item.get("postMarketClose").and_then(|v| v.as_str()) {
                        stock.post_market_close = parse_f64(post_close);
                    }
                    
                    if let Some(post_change) = item.get("postMarketChange").and_then(|v| v.as_str()) {
                        stock.post_market_change = parse_percentage(post_change);
                    }
                    
                    if let Some(post_volume) = item.get("postMarketVolume").and_then(|v| v.as_str()) {
                        stock.post_market_volume = parse_large_number(post_volume);
                    }
                },
                TabType::Valuation => {
                    // 估值标签页数据
                    if let Some(mcap_perf) = item.get("marketCapPerf1y").and_then(|v| v.as_str()) {
                        stock.market_cap_perf_1y = parse_percentage(mcap_perf);
                    }
                    
                    if let Some(peg) = item.get("pegRatio").and_then(|v| v.as_str()) {
                        stock.peg_ratio = parse_f64(peg);
                    }
                    
                    if let Some(ps) = item.get("priceToSales").and_then(|v| v.as_str()) {
                        stock.price_to_sales = parse_f64(ps);
                    }
                    
                    if let Some(pb) = item.get("priceToBook").and_then(|v| v.as_str()) {
                        stock.price_to_book = parse_f64(pb);
                    }
                    
                    if let Some(pcf) = item.get("priceToCashFlow").and_then(|v| v.as_str()) {
                        stock.price_to_cash_flow = parse_f64(pcf);
                    }
                    
                    if let Some(pfcf) = item.get("priceToFreeCashFlow").and_then(|v| v.as_str()) {
                        stock.price_to_free_cash_flow = parse_f64(pfcf);
                    }
                    
                    if let Some(pc) = item.get("priceToCash").and_then(|v| v.as_str()) {
                        stock.price_to_cash = parse_f64(pc);
                    }
                    
                    if let Some(ev) = item.get("enterpriseValue").and_then(|v| v.as_str()) {
                        stock.enterprise_value = parse_large_number(ev);
                    }
                    
                    if let Some(ev_rev) = item.get("evToRevenue").and_then(|v| v.as_str()) {
                        stock.ev_to_revenue = parse_f64(ev_rev);
                    }
                    
                    if let Some(ev_ebit) = item.get("evToEbit").and_then(|v| v.as_str()) {
                        stock.ev_to_ebit = parse_f64(ev_ebit);
                    }
                    
                    if let Some(ev_ebitda) = item.get("evToEbitda").and_then(|v| v.as_str()) {
                        stock.ev_to_ebitda = parse_f64(ev_ebitda);
                    }
                },
                TabType::Dividends => {
                    // 股利标签页数据
                    if let Some(dps_yearly) = item.get("dividendsPerShareYearly").and_then(|v| v.as_str()) {
                        stock.dividends_per_share_yearly = parse_f64(dps_yearly);
                    }
                    
                    if let Some(dps_quarterly) = item.get("dividendsPerShareQuarterly").and_then(|v| v.as_str()) {
                        stock.dividends_per_share_quarterly = parse_f64(dps_quarterly);
                    }
                    
                    if let Some(payout_ratio) = item.get("dividendPayoutRatio").and_then(|v| v.as_str()) {
                        stock.dividend_payout_ratio = parse_percentage(payout_ratio);
                    }
                    
                    if let Some(dps_growth) = item.get("dividendsPerShareGrowth").and_then(|v| v.as_str()) {
                        stock.dividends_per_share_growth = parse_percentage(dps_growth);
                    }
                    
                    if let Some(continuous_payout) = item.get("continuousDividendPayout").and_then(|v| v.as_str()) {
                        stock.continuous_dividend_payout = parse_large_number(continuous_payout);
                    }
                    
                    if let Some(continuous_growth) = item.get("continuousDividendGrowth").and_then(|v| v.as_str()) {
                        stock.continuous_dividend_growth = parse_large_number(continuous_growth);
                    }
                },
                TabType::Profitability => {
                    // 盈利能力标签页数据
                    if let Some(gm) = item.get("grossMargin").and_then(|v| v.as_str()) {
                        stock.gross_margin = parse_percentage(gm);
                    }
                    
                    if let Some(om) = item.get("operatingMargin").and_then(|v| v.as_str()) {
                        stock.operating_margin = parse_percentage(om);
                    }
                    
                    if let Some(pm) = item.get("profitMargin").and_then(|v| v.as_str()) {
                        stock.profit_margin = parse_percentage(pm);
                    }
                    
                    if let Some(purem) = item.get("pureMargin").and_then(|v| v.as_str()) {
                        stock.pure_margin = parse_percentage(purem);
                    }
                    
                    if let Some(fcfm) = item.get("freeCashFlowMargin").and_then(|v| v.as_str()) {
                        stock.free_cash_flow_margin = parse_percentage(fcfm);
                    }
                    
                    if let Some(roi) = item.get("roi").and_then(|v| v.as_str()) {
                        stock.roi = parse_percentage(roi);
                    }
                    
                    if let Some(roe) = item.get("roe").and_then(|v| v.as_str()) {
                        stock.roe = parse_percentage(roe);
                    }
                    
                    if let Some(roic) = item.get("roic").and_then(|v| v.as_str()) {
                        stock.roic = parse_percentage(roic);
                    }
                    
                    if let Some(rd) = item.get("rdRatio").and_then(|v| v.as_str()) {
                        stock.rd_ratio = parse_percentage(rd);
                    }
                    
                    if let Some(sga) = item.get("sgaRatio").and_then(|v| v.as_str()) {
                        stock.sga_ratio = parse_percentage(sga);
                    }
                },
                TabType::IncomeStatement => {
                    // 损益表标签页数据
                    if let Some(total_rev) = item.get("totalRevenue").and_then(|v| v.as_str()) {
                        stock.total_revenue = parse_large_number(total_rev);
                    }
                    
                    if let Some(rev_growth) = item.get("revenueGrowth").and_then(|v| v.as_str()) {
                        stock.revenue_growth = parse_percentage(rev_growth);
                    }
                    
                    if let Some(gross_prof) = item.get("grossProfit").and_then(|v| v.as_str()) {
                        stock.gross_profit = parse_large_number(gross_prof);
                    }
                    
                    if let Some(op_income) = item.get("operatingIncome").and_then(|v| v.as_str()) {
                        stock.operating_income = parse_large_number(op_income);
                    }
                    
                    if let Some(net_inc) = item.get("netIncome").and_then(|v| v.as_str()) {
                        stock.net_income = parse_large_number(net_inc);
                    }
                    
                    if let Some(ebitda) = item.get("ebitda").and_then(|v| v.as_str()) {
                        stock.ebitda = parse_large_number(ebitda);
                    }
                    
                    if let Some(eps_dil) = item.get("epsDiluted").and_then(|v| v.as_str()) {
                        stock.eps_diluted = parse_f64(eps_dil);
                    }
                    
                    if let Some(eps_growth) = item.get("epsDilutedGrowth").and_then(|v| v.as_str()) {
                        stock.eps_diluted_growth = parse_percentage(eps_growth);
                    }
                },
                TabType::BalanceSheet => {
                    // 资产负债表标签页数据
                    if let Some(tot_assets) = item.get("totalAssets").and_then(|v| v.as_str()) {
                        stock.total_assets = parse_large_number(tot_assets);
                    }
                    
                    if let Some(curr_assets) = item.get("totalCurrentAssets").and_then(|v| v.as_str()) {
                        stock.total_current_assets = parse_large_number(curr_assets);
                    }
                    
                    if let Some(cash) = item.get("cashAndShortTerm").and_then(|v| v.as_str()) {
                        stock.cash_and_short_term = parse_large_number(cash);
                    }
                    
                    if let Some(liabilities) = item.get("totalLiabilities").and_then(|v| v.as_str()) {
                        stock.total_liabilities = parse_large_number(liabilities);
                    }
                    
                    if let Some(tot_debt) = item.get("totalDebt").and_then(|v| v.as_str()) {
                        stock.total_debt = parse_large_number(tot_debt);
                    }
                    
                    if let Some(net_debt) = item.get("netDebt").and_then(|v| v.as_str()) {
                        stock.net_debt = parse_large_number(net_debt);
                    }
                    
                    if let Some(equity) = item.get("totalEquity").and_then(|v| v.as_str()) {
                        stock.total_equity = parse_large_number(equity);
                    }
                    
                    if let Some(curr_ratio) = item.get("currentRatio").and_then(|v| v.as_str()) {
                        stock.current_ratio = parse_f64(curr_ratio);
                    }
                    
                    if let Some(quick_ratio) = item.get("quickRatio").and_then(|v| v.as_str()) {
                        stock.quick_ratio = parse_f64(quick_ratio);
                    }
                    
                    if let Some(debt_equity) = item.get("debtToEquity").and_then(|v| v.as_str()) {
                        stock.debt_to_equity = parse_f64(debt_equity);
                    }
                    
                    if let Some(cash_debt) = item.get("cashToDebt").and_then(|v| v.as_str()) {
                        stock.cash_to_debt = parse_f64(cash_debt);
                    }
                },
                TabType::CashFlow => {
                    // 现金流标签页数据
                    if let Some(op_cf) = item.get("operatingCashFlow").and_then(|v| v.as_str()) {
                        stock.operating_cash_flow = parse_large_number(op_cf);
                    }
                    
                    if let Some(inv_cf) = item.get("investingCashFlow").and_then(|v| v.as_str()) {
                        stock.investing_cash_flow = parse_large_number(inv_cf);
                    }
                    
                    if let Some(fin_cf) = item.get("financingCashFlow").and_then(|v| v.as_str()) {
                        stock.financing_cash_flow = parse_large_number(fin_cf);
                    }
                    
                    if let Some(free_cf) = item.get("freeCashFlow").and_then(|v| v.as_str()) {
                        stock.free_cash_flow = parse_large_number(free_cf);
                    }
                    
                    if let Some(capex) = item.get("capitalExpenditures").and_then(|v| v.as_str()) {
                        stock.capital_expenditures = parse_large_number(capex);
                    }
                },
                TabType::Technicals => {
                    // 技术指标标签页数据
                    if let Some(tech_rating) = item.get("technicalRating").and_then(|v| v.as_str()) {
                        stock.technical_rating = Rating::from_str(tech_rating);
                    }
                    
                    if let Some(ma_rating) = item.get("maRating").and_then(|v| v.as_str()) {
                        stock.ma_rating = Rating::from_str(ma_rating);
                    }
                    
                    if let Some(osc_rating) = item.get("oscillatorsRating").and_then(|v| v.as_str()) {
                        stock.oscillators_rating = Rating::from_str(osc_rating);
                    }
                    
                    if let Some(rsi14) = item.get("rsi14").and_then(|v| v.as_str()) {
                        stock.rsi_14 = parse_f64(rsi14);
                    }
                    
                    if let Some(mom10) = item.get("momentum10").and_then(|v| v.as_str()) {
                        stock.momentum_10 = parse_f64(mom10);
                    }
                    
                    if let Some(ao) = item.get("awesomeOscillator").and_then(|v| v.as_str()) {
                        stock.awesome_oscillator = parse_f64(ao);
                    }
                    
                    if let Some(cci20) = item.get("cci20").and_then(|v| v.as_str()) {
                        stock.cci_20 = parse_f64(cci20);
                    }
                    
                    if let Some(stoch_k) = item.get("stochasticK").and_then(|v| v.as_str()) {
                        stock.stochastic_k = parse_f64(stoch_k);
                    }
                    
                    if let Some(stoch_d) = item.get("stochasticD").and_then(|v| v.as_str()) {
                        stock.stochastic_d = parse_f64(stoch_d);
                    }
                    
                    if let Some(pattern) = item.get("candlestickPattern").and_then(|v| v.as_str()) {
                        stock.candlestick_pattern = pattern.to_string();
                    }
                }
            }
            
            stocks.push(stock);
        }
    }
    
    Ok(stocks)
} 