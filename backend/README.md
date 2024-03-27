# Aenn backend

Aennのバックエンド

## 初期設定

1. Mysqlのインストール
2. sqlxのコマンドラインをインストール

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

3. `.env`ファイルを作成し、次のように書き込む。[]の中は自分の環境に合わせて書き換える

```.env
DATABASE_URL=mysql://[user]:[password]@[host]:[port]/Aenn
```

4. databaseの作成

```bash
sqlx db create
```

5. Rustの実行

```bash
cargo run
```
