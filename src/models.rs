// models.rs
// 本模块定义了 Todo 应用的数据模型，包括任务结构体及相关查询参数结构体。

use actix_web::web;
use serde::{Deserialize, Serialize};

/// Todo 任务结构体
///
/// 字段说明：
/// - `id`: Option\<i32\> - 任务唯一标识，新增时可为空
/// - `title`: String - 任务标题
/// - `completed`: bool - 是否已完成
/// - `category`: String - 任务所属分类
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub completed: bool,
    pub category: String
}

/// 实现从 web::Json<Todo> 到 Todo 的转换
impl From<web::Json<Todo>> for Todo {
    fn from(todo: web::Json<Todo>) -> Self {
        Todo {
            id: todo.id,
            title: todo.title.clone(),
            completed: todo.completed,
            category: todo.category.clone()
        }
    }
}

/// 搜索查询参数结构体
///
/// 字段说明：
/// - `query`: String - 搜索关键词
/// - `category`: Option\<String\> - 可选分类过滤
#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub category: Option<String>,
}

/// 获取任务查询参数结构体
///
/// 字段说明：
/// - `category`: Option<String> - 可选分类过滤
#[derive(Deserialize)]
pub struct FetchQuery {
    pub category: Option<String>,
}