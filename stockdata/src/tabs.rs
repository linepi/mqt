#[derive(Debug, Clone, Copy)]
pub enum TabType {
    Overview,      // 概览
    Performance,   // 表现
    ExtendedHours, // 延长时段
    Valuation,     // 估值
    Dividends,     // 股利
    Profitability, // 盈利能力
    IncomeStatement, // 损益表
    BalanceSheet,  // 资产负债表
    CashFlow,      // 现金流
    Technicals,    // 技术指标
}

impl TabType {
    pub fn id(&self) -> &'static str {
        match self {
            TabType::Overview => "overview",
            TabType::Performance => "performance",
            TabType::ExtendedHours => "extendedHours",
            TabType::Valuation => "valuation",
            TabType::Dividends => "dividends",
            TabType::Profitability => "profitability",
            TabType::IncomeStatement => "incomeStatement",
            TabType::BalanceSheet => "balanceSheet",
            TabType::CashFlow => "cashFlow",
            TabType::Technicals => "technicals",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TabType::Overview => "概览",
            TabType::Performance => "表现",
            TabType::ExtendedHours => "延长时段",
            TabType::Valuation => "估值",
            TabType::Dividends => "股利",
            TabType::Profitability => "盈利能力",
            TabType::IncomeStatement => "损益表",
            TabType::BalanceSheet => "资产负债表",
            TabType::CashFlow => "现金流",
            TabType::Technicals => "技术指标",
        }
    }

    pub fn all() -> Vec<TabType> {
        vec![
            TabType::Overview,
            TabType::Performance,
            TabType::ExtendedHours,
            TabType::Valuation,
            TabType::Dividends,
            TabType::Profitability,
            TabType::IncomeStatement,
            TabType::BalanceSheet,
            TabType::CashFlow,
            TabType::Technicals,
        ]
    }
} 