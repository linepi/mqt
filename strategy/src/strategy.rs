use crate::models::{StrategyParams, StrategyType, Signal, SignalAction, MarketData};
use log::info;
use std::collections::HashMap;

// 策略特征
pub trait Strategy {
    fn name(&self) -> &str;
    fn strategy_type(&self) -> StrategyType;
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn generate_signals(&self, data: &MarketData) -> Vec<Signal>;
}

// 动量策略
pub struct MomentumStrategy {
    params: StrategyParams,
    lookback_period: i32,
    threshold: f64,
}

impl MomentumStrategy {
    pub fn new(name: &str) -> Self {
        let mut params = StrategyParams::new(name.to_string(), StrategyType::Momentum);
        
        // 设置默认参数
        let _ = params.set_param("lookback_period", 20);
        let _ = params.set_param("threshold", 0.05);
        
        Self {
            params,
            lookback_period: 20,
            threshold: 0.05,
        }
    }
}

impl Strategy for MomentumStrategy {
    fn name(&self) -> &str {
        &self.params.name
    }
    
    fn strategy_type(&self) -> StrategyType {
        self.params.strategy_type
    }
    
    fn is_enabled(&self) -> bool {
        self.params.enabled
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.params.enabled = enabled;
    }
    
    fn generate_signals(&self, data: &MarketData) -> Vec<Signal> {
        info!("生成动量策略信号");
        
        // 这里只是创建一个模拟的信号
        // 实际应用中需要实现真正的策略逻辑
        
        let mut signals = Vec::new();
        
        // 假设根据股票涨幅生成信号
        for (code, snapshot) in &data.stocks {
            if snapshot.change_percent > self.threshold {
                signals.push(Signal {
                    code: code.clone(),
                    timestamp: data.timestamp,
                    action: SignalAction::Buy,
                    price: Some(snapshot.price),
                    amount: Some(100.0),
                    reason: format!("涨幅超过阈值 {}%", self.threshold * 100.0),
                    strength: snapshot.change_percent / 10.0, // 信号强度根据涨幅计算
                });
            } else if snapshot.change_percent < -self.threshold {
                signals.push(Signal {
                    code: code.clone(),
                    timestamp: data.timestamp,
                    action: SignalAction::Sell,
                    price: Some(snapshot.price),
                    amount: Some(100.0),
                    reason: format!("跌幅超过阈值 {}%", self.threshold * 100.0),
                    strength: -snapshot.change_percent / 10.0,
                });
            }
        }
        
        signals
    }
}

// 均值回归策略
pub struct MeanReversionStrategy {
    params: StrategyParams,
    ma_period: i32,
    std_dev_multiplier: f64,
}

impl MeanReversionStrategy {
    pub fn new(name: &str) -> Self {
        let mut params = StrategyParams::new(name.to_string(), StrategyType::MeanReversion);
        
        // 设置默认参数
        let _ = params.set_param("ma_period", 20);
        let _ = params.set_param("std_dev_multiplier", 2.0);
        
        Self {
            params,
            ma_period: 20,
            std_dev_multiplier: 2.0,
        }
    }
}

impl Strategy for MeanReversionStrategy {
    fn name(&self) -> &str {
        &self.params.name
    }
    
    fn strategy_type(&self) -> StrategyType {
        self.params.strategy_type
    }
    
    fn is_enabled(&self) -> bool {
        self.params.enabled
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.params.enabled = enabled;
    }
    
    fn generate_signals(&self, data: &MarketData) -> Vec<Signal> {
        info!("生成均值回归策略信号");
        
        // 这里只是创建一个模拟的信号
        // 实际应用中需要实现真正的策略逻辑
        
        let mut signals = Vec::new();
        
        // 假设根据股票价格与均值的偏离生成信号
        for (code, snapshot) in &data.stocks {
            // 假设当前价格偏离均值过大时生成信号
            if snapshot.price > snapshot.price * 1.1 { // 价格高于均值10%
                signals.push(Signal {
                    code: code.clone(),
                    timestamp: data.timestamp,
                    action: SignalAction::Sell,
                    price: Some(snapshot.price),
                    amount: Some(100.0),
                    reason: "价格高于均值过多".to_string(),
                    strength: 0.7,
                });
            } else if snapshot.price < snapshot.price * 0.9 { // 价格低于均值10%
                signals.push(Signal {
                    code: code.clone(),
                    timestamp: data.timestamp,
                    action: SignalAction::Buy,
                    price: Some(snapshot.price),
                    amount: Some(100.0),
                    reason: "价格低于均值过多".to_string(),
                    strength: 0.7,
                });
            }
        }
        
        signals
    }
}

// 策略工厂
pub struct StrategyFactory {
    strategies: HashMap<String, Box<dyn Strategy>>,
}

impl StrategyFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            strategies: HashMap::new(),
        };
        
        // 注册默认策略
        factory.register_strategy(Box::new(MomentumStrategy::new("动量策略")));
        factory.register_strategy(Box::new(MeanReversionStrategy::new("均值回归策略")));
        
        factory
    }
    
    pub fn register_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategies.insert(strategy.name().to_string(), strategy);
    }
    
    pub fn get_strategy(&self, name: &str) -> Option<&Box<dyn Strategy>> {
        self.strategies.get(name)
    }
    
    pub fn get_strategy_names(&self) -> Vec<String> {
        self.strategies.keys().cloned().collect()
    }
} 