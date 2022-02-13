# rust-web-server

rust + actix-web + actix-rt でのWebAPIサーバ

## SetUp .env

プロジェクト直下に`.env`を作成

|変数|設定値|
|:-|:-|
|SERVER_ADDRESS|アプリがListenするホスト（＋PORT）|
|CORS_ALLOWED_ORIGIN|CORSで許可するホスト（＋PORT)|
|DATABASE_HOST|データベース接続ホスト|
|DATABASE_PORT|データベース接続PORT|
|DATABASE_USER|データベース接続ユーザ名|
|DATABASE_PASS|データベース接続パスワード|
|DATABASE_NAME|データベース接続DB|
|DATABASE_POOL_SIZE|データベースコネクションプールサイズ|

```
SERVER_ADDRESS=localhost:5555
CORS_ALLOWED_ORIGIN=http://localhost:3000
DATABASE_HOST=127.0.0.1
DATABASE_PORT=3306
DATABASE_USER=root
DATABASE_PASS=mysql
DATABASE_NAME=test
DATABASE_POOL_SIZE=4
```

## Run

```
cargo run
```

## Build

```
cargo build
```

## curl

```
curl -X POST --data-urlencode 'text=hello' localhost:5555
```

```
curl -X POST -H  "Content-Type: application/x-www-form-urlencoded" -d 'text=Hello World!!' localhost:5555
```

```
curl -X POST -H  "Content-Type: application/json" -d '{"message":"Hi!John"}' localhost:5555/api/v1/hello
```

### CORS

CORSの設定確認(OK)(404はエンドポイントを宣言(例`#[head("/")]`)しAppに登録することで解消可能)

解消前(404)

```
curl -H "Origin: http://localhost:3000" --head http://localhost:5555

HTTP/1.1 404 Not Found
content-length: 0
access-control-allow-origin: http://localhost:3000
vary: Origin
date: Thu, 27 Jan 2022 04:57:00 GMT
```

解消後(200)

```
curl -H "Origin: http://localhost:3000" --head http://localhost:5555
HTTP/1.1 200 OK
content-length: 14
access-control-allow-origin: http://localhost:3000
vary: Origin
date: Sun, 13 Feb 2022 01:20:53 GMT
```

CORSの設定確認(Error)

```
curl -H "Origin: http://localhost:3001" --head http://localhost:5555

HTTP/1.1 400 Bad Request
content-length: 42
date: Thu, 27 Jan 2022 04:57:40 GMT
```
## Install Memo

`cargo add`の際に`error: no such subcommand:`で怒られる場合は`cargo install cargo-edit`を行う

```
cargo new rust-web-server
cd ./rust-web-server
cargo add actix-web actix-rt
cargo add actix-cors
```

エラーハンドリング
```
cargo add thiserror
```

テンプレートエンジン
```
cargo add askama
```

DB
```
cargo add r2d2-mysql
cargo add mysql
cargo add r2d2
```

dotenv
```
cargo add dotenv
```

### Cargo.toml

以下のバージョン構成だとエラーになるため

```
actix-rt = "2.6.0"
actix-web = "3.3.3"
```

`actix-web = "4.0.0-beta.3"` を設定

```
actix-web = "4.0.0-beta.3"
actix-rt = "2.6.0"
```

`cors`が`the trait  Transform<actix_web::app_service::AppRouting, ServiceRequest> is not implemented for Cors`で怒られるため

```
actix-cors = "0.5.4"
```

`actix-cors = "0.6.0-beta.4"`を設定する

```
actix-cors = "0.6.0-beta.4"
```

`builder`でエラーになるため`mysql = "22.0.0"` -> `mysql = "21.0.0"`を設定する

```
mysql = "21.0.0"
```