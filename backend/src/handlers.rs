/**
 * Todo に対する CRUD 操作を扱う
 */
use crate::models::{Todo, TodoList};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// GET /todos
// すべての TODO を取得する
async fn get_todos() -> impl Responder {
    let todo_list = TodoList::new();
    let todos = todo_list.get_all();
    HttpResponse::Ok().json(todos)
}

// POST /todos
#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
}

// 新しい TODO を作成する
async fn create_todo(new_todo: web::Json<CreateTodoRequest>) -> impl Responder {
    let mut todo_list = TodoList::new();
    let created_todo = todo_list.add(new_todo.title.clone());
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
        let mut todo_list = TodoList::new();

        // id から todo を更新
        let updated_todo = todo_list.get_mut(&id);
        if let Some(todo) = updated_todo {
            if let Some(title) = &update_todo.title {
                todo.title = title.clone();
            }
            if let Some(completed) = &update_todo.completed {
                todo.completed = completed.clone();
            }
            HttpResponse::Ok().json(todo)
        } else {
            HttpResponse::NotFound().finish()
        }
}

// DELETE /todos/{id}
// 既存の TODO を削除する
async fn delete_todo(id: web::Path<i32>) -> impl Responder {
    let mut todo_list = TodoList::new();
    todo_list.delete(&id);

    HttpResponse::Ok().finish()
}
