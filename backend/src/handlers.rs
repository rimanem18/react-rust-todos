/**
 * Todo に対する CRUD 操作を扱う
 */
use crate::models::{Todo, NewTodo};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// GET /todos
// すべての TODO を取得する
async fn get_todos() -> impl Responder {
    let todos = Todo::get_all();
    HttpResponse::Ok().json(todos)
}

// POST /todos
#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
}

// 新しい TODO を作成する
async fn create_todo(new_todo: web::Json<CreateTodoRequest>) -> impl Responder {
    let new_todo = NewTodo {
        title: new_todo.title.clone(),
        ..Default::default()
    };
    let created_todo = Todo::create(new_todo).await;
    HttpResponse::Created().json(created_todo)
}

// PUT /todos/{id}
#[derive(Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    completed: Option<bool>,
}

// 既存の TODO を更新する
async fn update_todo(
        id: web::Path<i32>,
        update_todo: web::Json<UpdateTodoRequest>,
    ) -> impl Responder {
        let mut todo = Todo::get(id.into_inner()).await.unwrap();
        if let Some(title) = update_todo.title {
            todo.title = title;
        }
        if let Some(completed) = update_todo.completed {
            todo.completed = completed;
        }
        let updated_todo = Todo::update().await;
        HttpResponse::Ok().json(updated_todo)
}

// DELETE /todos/{id}
// 既存の TODO を削除する
async fn delete_todo(id: web::Path<i32>) -> impl Responder {
    let num_deleted = Todo::delete(id.into_inner()).await;
    if num_deleted > 0 {
        HttpResponse::NoContent().finish()
     } else {
        HttpResponse::NotFound().finish()
    }
}
