use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}

mod date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

// TodoList の 構造の定義
// Todo を格納するリスト
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

// Todo のメソッド
impl Todo {
    pub fn new(title: String) -> Todo {
        let now = Utc::now();
        Todo {
            id:i32::default(),
            title,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }
}

// TodoList のメソッド
impl TodoList {
    // TodoList のインスタンス化
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    // Todo の追加
    pub fn add(&mut self, title: String) ->  Todo {
        let todo = Todo::new(title);
        self.todos.push(todo);
        // 挿入した todo を返す
        todo
    }

    // Todo をすべて取得
    pub fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    // ID に基づいて Todo の取得
    pub fn get(&self, id: i32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    // Todo を変更するために Todo のミュータブルな参照を返す
    pub fn get_mut(&mut self, id: &i32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == *id)
    }

    // Todo の削除
    pub fn delete(&mut self, id: &i32) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id == *id)?;
        Some(self.todos.remove(index))
    }
}
