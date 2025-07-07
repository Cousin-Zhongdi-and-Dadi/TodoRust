// main.rs
// 应用程序入口，负责初始化数据库连接、配置 CORS、注册路由并启动 Actix Web 服务器

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod handlers;
mod models;
mod routers;

use routers::todo_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载 .env 文件中的环境变量
    dotenv::dotenv().ok();
    // 获取数据库连接字符串，若未设置则 panic
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid number"); // 转换端口号为 u16

    // 创建 MySQL 数据库连接池，最大连接数为 5
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Starting server at http://{}:{}", base_url, port);

    // 启动 Actix Web HTTP 服务器
    HttpServer::new(move || {
        App::new()
            // 配置 CORS，允许所有来源和方法（生产环境建议收紧策略）
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            // 注入数据库连接池到应用数据
            .app_data(web::Data::new(pool.clone()))
            // 注册 todo 路由
            .configure(todo_routes)
    })
        // 绑定到本地 8080 端口
        .bind((base_url, port))?
        .run()
        .await
}