// handlers.rs
// 该模块实现了 Todo 应用的所有请求处理逻辑，包括创建、查询、更新、删除和分类获取等操作。

use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use super::models::{FetchQuery, SearchQuery, Todo};

/// 创建新的 Todo
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
/// - `new_todo`: web::Json<Todo> - 需要新建的 Todo 对象
///
/// # 返回
/// - `HttpResponse`：创建结果的响应
pub async fn create_todo(
    pool: web::Data<MySqlPool>,
    new_todo: web::Json<Todo>,
) -> HttpResponse {
    // 插入新任务，若分类为空则使用“默认”
    let result = sqlx::query!(
        "INSERT INTO todo (title, completed, category) VALUES (?, ?, ?)",
        new_todo.title,
        if new_todo.completed { 1 } else { 0 },
        if new_todo.category.is_empty() {
            "默认"
        } else {
            &new_todo.category
        }
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Todo created successfully"),
        Err(e) => {
            eprintln!("Error creating todo: {}", e);
            HttpResponse::InternalServerError().body("Failed to create todo")
        }
    }
}

/// 获取所有 Todo 列表
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
/// - `query`: web::Query<FetchQuery> - 查询参数，包含可选的分类过滤
///
/// # 返回
/// - `HttpResponse`: 包含 Todo 列表的响应
pub async fn get_all_todos(
    pool: web::Data<MySqlPool>,
    query: web::Query<FetchQuery>,
) -> HttpResponse {
    // 判断是否有分类过滤
    let category_filter = query.category.clone().unwrap_or_else(|| "all".to_string());
    let result = if category_filter == "all" {
        sqlx::query_as!(
            Todo,
            "SELECT id as `id: i32`, title, completed as `completed: bool`, category FROM todo"
        )
            .fetch_all(pool.get_ref())
            .await
    } else {
        sqlx::query_as!(
            Todo,
            "SELECT id as `id: i32`, title, completed as `completed: bool`, category FROM todo WHERE category = ?",
            category_filter
        )
            .fetch_all(pool.get_ref())
            .await
    };
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            eprintln!("Error fetching todos: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch todos")
        }
    }
}

/// 更新指定 ID 的 Todo
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
/// - `id`: web::Path<i32> - Todo 的 ID
/// - `todo`: web::Json<Todo> - 需要更新的 Todo 内容
///
/// # 返回
/// - `HttpResponse`: 更新结果的响应
pub async fn update_todo(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    // 根据 ID 更新任务内容
    let result = sqlx::query!(
        "UPDATE todo SET title = ?, completed = ?, category = ? WHERE id = ?",
        todo.title,
        if todo.completed { 1 } else { 0 },
        todo.category,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Todo updated successfully"),
        Err(e) => {
            eprintln!("Error updating todo: {}", e);
            HttpResponse::InternalServerError().body("Failed to update todo")
        }
    }
}

/// 删除指定 ID 的 Todo
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
/// - `id`: web::Path<i32> - Todo 的 ID
///
/// # 返回
/// - `HttpResponse`: 删除结果的响应
pub async fn delete_todo(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    // 根据 ID 删除任务
    let result = sqlx::query!("DELETE FROM todo WHERE id = ?", id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Todo deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting todo: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete todo")
        }
    }
}

/// 搜索 Todo
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
/// - `query`: web::Query<SearchQuery> - 查询参数，包含搜索关键词和可选的分类过滤
///
/// # 返回
/// - `HttpResponse`: 包含搜索结果的响应
pub async fn search_todos(
    pool: web::Data<MySqlPool>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    // 构造模糊查询关键词
    let search_term = format!("%{}%", query.query);
    let search_category = query.category.clone().unwrap_or_else(|| "all".to_string());
    let result = if search_category == "all" {
        sqlx::query_as!(
            Todo,
            "SELECT id as `id: i32`, title, completed as `completed: bool`, category FROM todo WHERE title LIKE ?",
            search_term
        )
            .fetch_all(pool.get_ref())
            .await
    } else {
        sqlx::query_as!(
            Todo,
            "SELECT id as `id: i32`, title, completed as `completed: bool`, category FROM todo WHERE title LIKE ? AND category = ?",
            search_term,
            search_category
        )
            .fetch_all(pool.get_ref())
            .await
    };

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            eprintln!("Error searching todos: {}", e);
            HttpResponse::InternalServerError().body("Failed to search todos")
        }
    }
}

/// 获取所有的分类
///
/// # 参数
/// - `pool`: web::Data<MySqlPool> - 数据库连接池
///
/// # 返回
/// - `HttpResponse`: 包含所有分类的响应
pub async fn get_all_categories(pool: web::Data<MySqlPool>) -> HttpResponse {
    // 查询所有去重后的分类
    let result = sqlx::query!("SELECT DISTINCT category FROM todo")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(categories) => {
            let categories: Vec<String> = categories.into_iter()
                .map(|c| c.category)
                .collect();
            HttpResponse::Ok().json(categories)
        },
        Err(e) => {
            eprintln!("Error fetching categories: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch categories")
        }
    }
}