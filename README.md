### init

```
# コンテナ起動
make up

# コンテナに入る
make app

# 再帰的に所有権変更
chown appuser:appuser /work/backend -R
```

### command
```
# directory rust-sample-app

# コンテナの破棄
make down

# キャッシュを使わずにコンテナリビルド
make rebuild
```

### server start
```
# directory app container /work/backend/server

# サーバー起動 & ホットリロード
cargo watch -x run
```
