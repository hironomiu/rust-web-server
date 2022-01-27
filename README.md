# rust-web-server

rust + actix-web + actix-rt でのWebAPIサーバ

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
curl -X POST -H  "Content-Type: application/json" -d '{"message":"Hi!John"}' localhost:5555/api/v1/hello
```

## Install Memo

`cargo add`の際に`error: no such subcommand:`で怒られる場合は`cargo install cargo-edit`を行う

```
cargo new rust-web-server
cd ./rust-web-server
cargo add actix-web actix-rt
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