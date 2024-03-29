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

4. mysqlでuserにarticlesの挿入と選択の権限の付与(面倒だったので、権限をすべて渡しており注意)

```bash
> sudo mysql -u root
mysql> GRANT INSERT, SELECT, UPDATE, DELETE, DROP ON *.* TO '[user]'@'[host]';
mysql> FLUSH PRIVILEGES;
```

5. データベースとテーブルの作成

```
> sqlx db create
> sqlx migrate run
```

6. Rustの実行

```bash
cargo run
```

## 初期データの削除

初期データを入れたくなかったら、migrationsファイルのINSERTを除く

## データベースの削除

```bash
sqlx db drop
```
