use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Todo の 構造を定義
// 各 Todo のデータを表す
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// TodoList の 構造の定義
// Todo を格納するリスト
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

// TodoList のメソッド
impl TodoList {
    // TodoList のインスタンス化
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    // Todo の追加
    pub fn add(&mut self, title: String) -> &Todo {
        let todo = Todo::new(title);
    }

    // ID に基づいて Todo の取得
    pub fn get(&self, id: &Uuid) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == *id)
    }

    // Todo を変更するために Todo のミュータブルな参照を返す
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == *id)
    }

    // Todo の削除
    pub fn delete(&mut self, id: &Uuid) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id == *id)?;
        Some(self.todos.remove(index))
    }
}
