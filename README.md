# async-graphql playground

Rust の GraphQL ライブラリ async-graphql の検証リポジトリ

## 構成情報

- Rust: 1.72.0
- async-graphql: 6.0.7
- axum: 0.6.20

## ファイル構成

```sh
.
├── README.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain
└── src/
    ├── main.rs
    ├── models.rs
    ├── mutation.rs
    ├── query.rs
    └── schema.rs
```

##　　ローカル環境構築

### パッケージ・ライブラリをインストールする

```sh
# パッケージのインストール
cargo install --path .

# ライブリロード用のライブラリをインストールする
cargo install cargo-watch
```

## async-graphql の起動

```sh
cargo watch -x run
```

### 動作確認

**取得リクエスト**

```sh
curl -s 'http://localhost:8000' \
  --data-raw '{ "query": "query { users { name, age } }" }'
```

**登録リクエスト**

```sh
curl -s 'http://localhost:8000' \
  --data-raw '{ "query": "mutation { addUser(name: \"carol\", age: 107) { name, age } }" }'
```

## ドキュメント

- [Introduction - Async-graphql Book](https://async-graphql.github.io/async-graphql/en/index.html)
- [GitHub - async-graphql](https://github.com/async-graphql/async-graphql)
