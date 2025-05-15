use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

pub struct ServerState {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            start_time: chrono::Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ServerStatus {
    pub status: String,
    pub uptime: String,
    pub version: String,
}

// 获取服务器状态
pub async fn get_status(data: web::Data<ServerState>) -> impl Responder {
    let now = chrono::Utc::now();
    let uptime = now - data.start_time;
    
    let status = ServerStatus {
        status: "running".to_string(),
        uptime: format!("{} days, {} hours, {} minutes", 
            uptime.num_days(),
            uptime.num_hours() % 24,
            uptime.num_minutes() % 60
        ),
        version: data.version.clone(),
    };
    
    HttpResponse::Ok().json(status)
}

// 健康检查
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
