# フェーズ4: タスク管理 Web API

## ゴール

axumフレームワークでREST APIを作り、SQLiteに永続化、JWT認証を実装する。

## 成果物の仕様

### エンドポイント

| メソッド | パス | 機能 |
|---|---|---|
| `POST`   | `/tasks`                | タスクを追加（add） |
| `GET`    | `/tasks`                | タスク一覧（list） — クエリで `?q=...` を渡すと検索（find） |
| `DELETE` | `/tasks/:id`            | タスクを削除 |
| `PATCH`  | `/tasks/:id`            | タスクを部分更新（update / done両方） |

> **REST設計の選択肢**: 学習用なので、リソース指向のシンプルな構成にしている。
> - 検索を別パスにする `GET /tasks/search?q=...` も可
> - 完了専用エンドポイント `POST /tasks/:id/done` のようなアクション系も実用的
> - フェーズ4で両者の使い分けを学ぶ価値あり

### 認証付きエンドポイント（フェーズ4-B）

| メソッド | パス | 機能 |
|---|---|---|
| `POST` | `/auth/register` | ユーザー登録 |
| `POST` | `/auth/login`    | ログイン（JWT発行） |
| `*`    | `/tasks/*`       | JWT必須に変更 |

## 進め方

### フェーズ4-A: 認証なしのCRUD API
1. axum でサーバー起動
2. SQLite + sqlx で DB 接続
3. ハンドラ実装
4. エラー処理
5. テスト
6. 環境変数管理（`dotenvy`）

### フェーズ4-B: JWT認証の追加
1. ユーザーテーブル追加
2. パスワードハッシュ（**argon2id**）
3. JWT発行・検証
4. 認証ミドルウェア
5. 既存エンドポイントを認証必須に

## 使用するクレート

| クレート | 用途 | 補足 |
|---|---|---|
| `axum`              | Webフレームワーク | 2026年時点のRustデファクト |
| `tokio`             | 非同期ランタイム | |
| `sqlx`              | SQLiteアクセス | コンパイル時クエリ検証 |
| `serde` / `serde_json` | JSONシリアライズ | |
| `tower` / `tower-http` | ミドルウェア | CORSなど |
| `tracing`           | ロギング | |
| `jsonwebtoken`      | JWT | |
| `argon2`            | パスワードハッシュ（argon2id） | OWASP 2025推奨。bcryptはレガシー |
| `dotenvy`           | 環境変数管理 | `dotenv`はメンテ停止のため `dotenvy` を使う |
| `validator`         | リクエストバリデーション | `#[derive(Validate)]`。FastAPIの Pydantic 相当 |
| `anyhow` / `thiserror` | エラー処理 | アプリ層は`anyhow`、エラー型は`thiserror` |
| `axum-test` または `tower::ServiceExt::oneshot` | 統合テスト | |

## 学習ポイント

- HTTPフレームワークの構造（axum の `Router`, `Handler`, `Extractor`）
- DB接続プールと async
- エラーをHTTPレスポンスに変換する設計（`thiserror` + `IntoResponse`）
- ミドルウェア（tower）
- JWT認証の標準的な実装パターン
- 環境変数による秘匿情報管理（**JWT秘密鍵は絶対にハードコードしない**）

## sqlx の運用知識

- `sqlx::query!` マクロを使うと **コンパイル時にDB接続が必要**
- CI環境などDBがない場所では:
  - `cargo sqlx prepare` でクエリメタデータを `.sqlx/` ディレクトリに保存
  - `SQLX_OFFLINE=true` でオフラインビルド可能
- 学習用ローカル開発では問題ないが、知っておく価値あり

## SQLiteの注意点

- 書き込みがシリアライズされる制約あり
- 学習用には十分だが、本番ではPostgreSQLへの移行を見据えた設計を意識

## Python（FastAPI）との対応表

| FastAPI / Flask | Rust相当 |
|---|---|
| Pydantic | `serde` + `validator` |
| `python-jose` | `jsonwebtoken` |
| `passlib[bcrypt]` | `argon2` |
| `python-dotenv` | `dotenvy` |
| `pytest` + `httpx` | `tokio::test` + `reqwest` または `axum-test` |
| SQLAlchemy | `sqlx`（よりSQL直書き寄り） |
