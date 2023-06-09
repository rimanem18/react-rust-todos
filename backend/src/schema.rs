/**
 * Diesel ORM のマイグレーションファイル
 * Todo のテーブル定義
 */
use diesel::table;


table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
