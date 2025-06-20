use super::handlers::*;
use actix_web::{ web};

/// 注册 Todo 相关的路由
///
/// # 参数
/// - `cfg`: &mut web::ServiceConfig - Actix Web 路由配置对象
pub fn todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/todos") // 定义 API 路径前缀
            .route("/", web::get().to(get_all_todos)) // 获取所有 Todo 列表
            .route("/", web::post().to(create_todo)) // 创建新的 Todo
            .route("/search", web::get().to(search_todos)) // 搜索 Todo
            .route("/{id}", web::put().to(update_todo)) // 更新 Todo
            .route("/{id}", web::delete().to(delete_todo)) // 删除 Todo
            .route("/categories", web::get().to(get_all_categories)) // 获取所有分类
    );
}