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

5. mysqlでuserにarticlesの挿入と選択の権限の付与

```bash
mysql> GRANT SELECT ON Aenn.articles TO '[user]'@'[host]';
mysql> GRANT INSERT ON Aenn.articles TO '[user]'@'[host]';
mysql> GRANT SELECT ON Aenn.comments TO '[user]'@'[host]';
mysql> GRANT INSERT ON Aenn.comments TO '[user]'@'[host]';
mysql> FLUSH PRIVILEGES;
```

6. Rustの実行

```bash
cargo run
```

7. (初期データを入れる場合)のArticleの挿入

```bash
mysql> USE Aenn
```

init_dataset.txtの中身の実行

```bash
mysql> SELECT * FROM articles;
```
