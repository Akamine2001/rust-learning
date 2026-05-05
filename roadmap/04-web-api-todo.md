# フェーズ4: タスク管理 Web API TODO

## フェーズ4-A: 認証なしのCRUD API

### プロジェクト初期化
- [ ] `cd practice && cargo new task-api` でプロジェクト作成（置き場は `practice/task-api/`）
- [ ] `Cargo.toml` に必要なクレートを追加（axum, tokio, sqlx, serde, tracing, dotenvy 等）
- [ ] サーバー起動の最小コードを動かす（"Hello, World!" を返す）

### 環境変数管理
- [ ] `.env` ファイルを作成（`DATABASE_URL` など）
- [ ] `.env` を `.gitignore` に追加
- [ ] `dotenvy` で起動時に読み込む

### DB セットアップ
- [ ] SQLite ファイルを作成
- [ ] `tasks` テーブルのマイグレーションを書く（`sqlx migrate add`）
- [ ] sqlx で DB 接続プールを作る
- [ ] アプリ起動時にマイグレーションを実行
- [ ] `cargo sqlx prepare` の運用を理解（CIでのオフラインビルド）

### モデル
- [ ] `Task` 構造体を定義
- [ ] sqlx の `FromRow` を実装
- [ ] リクエスト/レスポンス用のDTO型を定義
- [ ] `validator` でリクエストバリデーション（例: タイトル空文字禁止）

### ハンドラ実装
- [ ] `POST /tasks` (add)
- [ ] `GET /tasks` (list) — クエリ `?q=...` で検索対応（または `GET /tasks/search`）
- [ ] `DELETE /tasks/:id`
- [ ] `PATCH /tasks/:id`（updateとdoneを部分更新で）

### エラー処理
- [ ] エラー型を定義（`thiserror`）
- [ ] エラーをHTTPレスポンスに変換する `IntoResponse` 実装
- [ ] 適切なステータスコードを返す

### ロギング
- [ ] `tracing` を導入
- [ ] `tower-http::trace::TraceLayer` でリクエストログを出す

### CORS（必要に応じて）
- [ ] `tower-http::cors::CorsLayer` で設定

### テスト
- [ ] **統合テスト**: `axum-test` または `tower::ServiceExt::oneshot` でエンドポイントを叩く
  - 登録 → 一覧取得 → 削除 のシナリオ
  - エラーケース（存在しないID、不正なJSON）

### 動作確認
- [ ] curl または Postman で全エンドポイントを叩く

## フェーズ4-B: JWT認証の追加

### ユーザー管理
- [ ] `users` テーブルのマイグレーション
- [ ] `User` 構造体を定義
- [ ] `POST /auth/register`（パスワードを **argon2id** でハッシュ化して保存）
- [ ] `POST /auth/login`（パスワード検証 → JWT発行）

### JWT
- [ ] **JWT秘密鍵は環境変数経由で読み込む**（ハードコード禁止）
- [ ] JWT発行ヘルパーを書く（`jsonwebtoken`）
- [ ] JWT検証ヘルパーを書く
- [ ] Claims 構造体（`sub`, `exp` など）を定義

### 認証ミドルウェア
- [ ] axum のミドルウェアでJWT検証（`axum::middleware::from_fn` または `FromRequestParts` 実装）
- [ ] `Authorization: Bearer ...` ヘッダーから取り出す
- [ ] 検証済みユーザー情報をハンドラに渡す（`Extension` または カスタム Extractor）

### エンドポイント保護
- [ ] `/tasks/*` を認証必須に
- [ ] タスクをユーザー単位で管理（`tasks` テーブルに `user_id` を追加）

### テスト
- [ ] 認証なしで `/tasks` にアクセスして 401 が返ることを確認
- [ ] 登録 → ログイン → タスク作成 → 一覧取得 のフローを統合テストで確認

## 仕上げ
- [ ] README.md を書く
- [ ] APIドキュメント（簡易でOK）
- [ ] `cargo fmt` / `cargo clippy` で整形・lint

## フェーズ完了チェック
- [ ] 全エンドポイントが動作
- [ ] 認証フローが動作
- [ ] **学習プロジェクト完了！**
