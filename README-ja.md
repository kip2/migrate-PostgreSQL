[English](README.md)|[日本語](README-ja.md)

<h1 align="center"> migrate </h1>

<h2 align="center"> シンプルなマイグレーションアプリ </h2>

コンソールで実行できるシンプルなマイグレーションアプリケーションです。

# 前提

- 本アプリケーションはPostgreSQLの使用が前提となっています。
- DBの設定は.envを使用します。
- バイナリファイルの形では配布していませんので、各自の環境でビルドを行ってください。

# 環境設定

## ビルドの方法

Rustのインストールされた環境で以下を実行する。

```shell
cargo build --release
```

ビルドされた実行ファイルをディレクトリに配置してください。

ビルドされたファイルは以下のパスに生成されます。

```shell
./target/release/migrate
```

## DBのアクセス設定

DBアクセス情報の設定が必要になります。

以下の手順で行ってください。

1. .envファイルをビルドしたmigrationと同じディレクトリに配置する。
2. .envファイルに以下の形式で、DBアクセス設定を記入しください。

```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

# 事前準備

## マイグレーション管理用テーブルの作成

最初に、マイグレーションを管理するためのテーブルを作成します。

実行前に、PostgreSQLを使える環境、かつ、`.env`で接続の設定をしていることを確認してください。

```shell
./migrate -i

# もしくは
./migrate --init
```

コマンド実行後、DBに`migrations`という名前の、マイグレーション管理用のテーブルが作成されます。

## マイグレーションファイルの作成

実行するマイグレーションを定義するためのファイルを作成します。

```shell
./migrate -c

# もしくは
./migrate --create
```

コマンド実行後、以下のようなファイルが作成されます。

```shell
# up file
./Migrations/<YYYY-MM-DD>_<UNIX_TIME_STAMP>_up.sql
# ./Migrations/2000-01-01_1234567890_up.sql

# down file
./Migrations/<YYYY-MM-DD>_<UNIX_TIME_STAMP>_down.sql
# ./Migrations/2000-01-01_1234567890_down.sql
```

## 実行したいマイグレーションの設定

作成された`up file`と`down file`に、実行したいマイグレーションを記載します。

### up file

実行したいマイグレーションを記載します。

SQLとして実行できる文であれば、記載可能です。

複数のクエリの記述も可能です。

記載時の注意点として、それぞれのSQL文の末尾には、必ず`;`をつけてください。

一例として、以下のようなテーブル作成のSQL文などを書くとよいでしょう。

```sql
CREATE TABLE users (
                id BIGINT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

### down file

up fileに記載したSQL実行結果を、ロールバックするためのSQL文を記載します。

一例として、

- `up file`にテーブル作成のSQL文を定義。 -> `down file`に`up file`で作成したテーブルを削除するSQL文を定義。
- `up file`にデータをインサートするSQL文を定義。 -> `down file`に`up file`でインサートしたデータを削除するSQL文を定義。

以下の例では、`up file`で定義したテーブルを削除するSQL文を定義しています。

```sql
DROP TABLE users;
```

# マイグレーションの実行

環境設定、事前準備が完了した後、以下のコマンドでマイグレーションが実行されます。

```shell
./migrate
```

## マイグレーション対象ファイル

過去に実行したマイグレーションは`migrations`テーブルで管理されています。

新規にマイグレーションファイルを追加定義した場合は、新規追加したマイグレーションのみが実行されます。

一例として手順を示します。

```shell
# マイグレーションを実行
./migrate

# 新しいマイグレーションファイルを作成
./migrate --create

# マイグレーションファイルにSQL文を追加定義
# 記載は割愛

# 新規マイグレーションを実行
# 新規追加したマイグレーションのみが実行される
./migrate
```

# ロールバック

行ったマイグレーションを、特定の段階まで戻すことが可能です。

コマンドは以下のようになります。

```shell
# <n>は、いくつ前の段階に戻すかの回数を指定する
./migrate -r <n>
# もしくは
./migrate --rollback <n>

# 一例
# 2段階前の状態に戻す場合
./migrate -r 2
# もしくは
./migrate --rollback 2
```

また、可能な回数のロールバックのみを実行します。

2回マイグレーションを行っているDBの場合、2以上の数値を指定した場合は、2回のみロールバックが行われます(10や1000を指定しても2回のみ実行される)。

# help

コマンドについて困った時はヘルプを参照してください。

以下のコマンドでヘルプが参照可能です。

```shell
./migrate -h
# もしくは
./migrate --help
```

# LICENSE

[MIT LICENSE](https://github.com/kip2/sqcr/blob/main/LICENSE)

# AUTHOR

[kip2](https://github.com/kip2)
