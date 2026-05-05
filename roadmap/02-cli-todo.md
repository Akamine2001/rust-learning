# フェーズ2: タスク管理CLI TODO

## プロジェクト初期化
- [ ] `cd practice && cargo new task-cli` でプロジェクト作成（置き場は `practice/task-cli/`）
- [ ] `Cargo.toml` に必要なクレートを追加
- [ ] `Cargo.toml` のメタデータを整備（`description`, `license`, `version`）
- [ ] 動作確認（`cargo run`）

## データモデル
- [ ] `Task` 構造体を定義
- [ ] `derive` で `Debug`, `Clone`, `Serialize`, `Deserialize` を付与

## CLI引数のパース（clap v4 derive API）
- [ ] `#[derive(Parser)]` でルート構造体を定義
- [ ] `#[derive(Subcommand)]` で `add`, `list`, `delete`, `find`, `update`, `done` を定義
- [ ] 各サブコマンドの引数を定義

## データ永続化
- [ ] JSON保存・読込のヘルパー関数を作る
- [ ] 保存先パスを決める（学習用に `~/.task/tasks.json`）
- [ ] ファイルが存在しない時の初期化処理

## コマンド実装
- [ ] `add` 実装
- [ ] `list` 実装
- [ ] `delete` 実装
- [ ] `find` 実装
- [ ] `update` 実装
- [ ] `done` 実装

## エラー処理
- [ ] `anyhow` を導入（アプリ層のエラーは `anyhow::Result<T>`）
- [ ] エラーメッセージをユーザーに分かりやすく表示
- [ ] `thiserror` の使い所も学ぶ（ストレージ層など意味のあるエラー型を返したい所だけ）

## モジュール分割
- [ ] `model.rs`（データモデル）
- [ ] `storage.rs`（永続化）
- [ ] `commands/`（各コマンドの実装）

## ドキュメント
- [ ] 公開関数・構造体に `///` ドキュメンテーションコメントを書く
- [ ] `cargo doc --open` でドキュメント生成

## テスト
- [ ] **ユニットテスト**: `storage` モジュールのJSON往復をテスト
- [ ] **統合テスト**: `assert_cmd` + `predicates` でサブコマンドの実行結果を検証
  - `tests/` ディレクトリに統合テストを置く
  - `cargo add --dev assert_cmd predicates`

## コード品質
- [ ] `cargo fmt` で整形
- [ ] `cargo clippy` でlint（警告ゼロを目指す）

## 仕上げ
- [ ] README.md を書く（使い方）
- [ ] `cargo install --path .` でローカルインストールして使ってみる

## フェーズ完了チェック
- [ ] 全コマンドが動作する
- [ ] フェーズ3に進む準備完了
