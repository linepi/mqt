use std::env;
use log::info;
use std::io as std_io;

// 导入这些模块
mod models;  // 数据模型
mod tabs;    // 标签页类型
mod scraper; // 网页抓取
mod parser;  // 数据解析
mod io;      // 输入输出处理
mod scripts; // JavaScript脚本
mod server;  // HTTP服务器

#[actix_web::main]
async fn main() -> std_io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 获取端口，默认为8080
    let port = match env::var("PORT") {
        Ok(port_str) => port_str.parse::<u16>().unwrap_or(8080),
        Err(_) => 8080,
    };
    
    info!("股票数据服务启动，端口：{}", port);
    
    // 启动HTTP服务器
    server::start_server(port).await
}