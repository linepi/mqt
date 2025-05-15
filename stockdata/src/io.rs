use std::fs::{self, File};
use std::io::{self, Write, BufWriter};
use std::path::Path;
use chrono::Local;
use crate::models::StockData;
use std::collections::HashMap;
use crate::models::merge_stock_data;
use log::{info, error};

// 将股票数据保存为JSON文件
pub fn save_to_json(stocks: &[StockData], filename: &str) -> io::Result<()> {
    // 使用serde_json直接序列化完整对象, 确保所有字段都被保存
    let json_string = serde_json::to_string_pretty(stocks)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    // 写入文件, 使用UTF-8编码
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(json_string.as_bytes())?;
    
    Ok(())
}

// 从JSON文件读取股票数据
#[allow(dead_code)]
pub fn load_from_json(filename: &str) -> io::Result<Vec<StockData>> {
    let file_content = fs::read_to_string(filename)?;
    let stocks: Vec<StockData> = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(stocks)
}

// 创建输出目录
pub fn create_output_dir(dir: &str) -> io::Result<()> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

// 获取带有时间戳的文件名
pub fn get_timestamped_filename(dir: &str, prefix: &str, extension: &str) -> String {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    format!("{}/{}_{}{}",
            dir,
            prefix,
            timestamp,
            if extension.starts_with('.') { extension.to_string() } else { format!(".{}", extension) })
}

// 保存股票数据到文件, 自动生成文件名
pub fn save_stock_data(stocks: &[StockData]) -> io::Result<String> {
    // 创建输出目录
    let output_dir = "output";
    if let Err(e) = create_output_dir(output_dir) {
        error!("创建输出目录失败: {}, 将保存到当前目录", e);
    }
    
    // 生成带时间戳的文件名
    let json_filename = get_timestamped_filename(output_dir, "stock_data", "json");
    
    // 保存数据到文件
    match save_to_json(stocks, &json_filename) {
        Ok(_) => {
            info!("数据已保存到 {}", json_filename);
            Ok(json_filename)
        },
        Err(e) => {
            error!("保存JSON文件失败: {}", e);
            Err(e)
        }
    }
}

// 合并多个股票数据来源
pub fn merge_stock_data_sources(data_sources: &[Vec<StockData>]) -> Vec<StockData> {
    // 创建一个映射, 用股票代码做键
    let mut merged_stocks: HashMap<String, StockData> = HashMap::new();
    
    // 遍历所有数据源
    for source in data_sources {
        for stock in source {
            // 如果股票代码已经存在于映射中, 则合并数据
            if let Some(existing_stock) = merged_stocks.get_mut(&stock.code) {
                merge_stock_data(existing_stock, stock);
            } else {
                // 否则, 添加新的股票数据
                merged_stocks.insert(stock.code.clone(), stock.clone());
            }
        }
    }
    
    // 将HashMap转换为Vec
    merged_stocks.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merge_stock_data_sources() {
        // 创建测试数据
        let mut source1 = Vec::new();
        let mut source2 = Vec::new();
        let mut source3 = Vec::new();
        
        // 源1: 包含两支股票的基本信息
        let mut stock1 = StockData::default();
        stock1.code = "SH000001".to_string();
        stock1.name = "上证指数".to_string();
        stock1.price = 3000.0;
        stock1.change_percent = 1.5;
        source1.push(stock1);
        
        let mut stock2 = StockData::default();
        stock2.code = "SZ399001".to_string();
        stock2.name = "深证成指".to_string();
        stock2.price = 10000.0;
        stock2.change_percent = -0.8;
        source1.push(stock2);
        
        // 源2: 包含表现数据
        let mut stock1_perf = StockData::default();
        stock1_perf.code = "SH000001".to_string();
        stock1_perf.performance_1w = 2.5;
        stock1_perf.performance_1m = -1.2;
        stock1_perf.performance_1y = 15.3;
        source2.push(stock1_perf);
        
        // 源3: 包含估值数据
        let mut stock1_val = StockData::default();
        stock1_val.code = "SH000001".to_string();
        stock1_val.market_cap = 30000000000;
        stock1_val.pe_ratio = 15.2;
        
        let mut stock3 = StockData::default();
        stock3.code = "SH600000".to_string();
        stock3.name = "浦发银行".to_string();
        stock3.price = 10.5;
        stock3.market_cap = 300000000;
        source3.push(stock1_val);
        source3.push(stock3);
        
        // 合并数据
        let data_sources = vec![source1, source2, source3];
        let merged = merge_stock_data_sources(&data_sources);
        
        // 验证结果
        assert_eq!(merged.len(), 3); // 应该有3支股票
        
        // 查找并验证上证指数的数据
        if let Some(sh000001) = merged.iter().find(|s| s.code == "SH000001") {
            assert_eq!(sh000001.name, "上证指数");
            assert_eq!(sh000001.price, 3000.0);
            assert_eq!(sh000001.change_percent, 1.5);
            assert_eq!(sh000001.performance_1w, 2.5);
            assert_eq!(sh000001.performance_1m, -1.2);
            assert_eq!(sh000001.performance_1y, 15.3);
            assert_eq!(sh000001.market_cap, 30000000000);
            assert_eq!(sh000001.pe_ratio, 15.2);
        } else {
            panic!("未找到上证指数的数据");
        }
        
        // 验证浦发银行的数据
        if let Some(sh600000) = merged.iter().find(|s| s.code == "SH600000") {
            assert_eq!(sh600000.name, "浦发银行");
            assert_eq!(sh600000.price, 10.5);
            assert_eq!(sh600000.market_cap, 300000000);
        } else {
            panic!("未找到浦发银行的数据");
        }
    }
} 