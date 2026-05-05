# フェーズ2: タスク管理CLI

## ゴール

Rustでコマンドラインツールを作る経験を積み、フェーズ1で学んだ基礎を実戦投入する。

## 成果物の仕様

### コマンド一覧

| コマンド | 説明 | 例 |
|---|---|---|
| `add`    | タスクを追加 | `task add "牛乳を買う"` |
| `list`   | タスク一覧を表示 | `task list` |
| `delete` | タスクを削除 | `task delete 1` |
| `find`   | タスクを検索 | `task find "牛乳"` |
| `update` | タスクを更新 | `task update 1 "豆乳を買う"` |
| `done`   | タスクを完了状態にする | `task done 1` |

### データモデル

```rust
struct Task {
    id: u32,
    title: String,
    done: bool,
    created_at: DateTime,
}
```

### データ永続化

ローカルのJSONファイルに保存（学習用に `~/.task/tasks.json` を手書きで指定）。

> **本来のベストプラクティス**: クロスプラットフォーム対応のため `directories` クレートで `ProjectDirs` を使うのが Rust 界隈の慣習。本フェーズでは学習用に簡易化するが、覚えておきたい。

## 使用するクレート

| クレート | 用途 | 補足 |
|---|---|---|
| `clap` (v4, **derive API**) | CLI引数のパース | `#[derive(Parser)]` でサブコマンド構成を定義 |
| `serde` / `serde_json` | JSONシリアライズ・デシリアライズ | デファクト |
| `chrono` | 日時の扱い | デファクト。代替として `time` / `jiff` も台頭中 |
| `anyhow` | アプリケーション層のエラー処理 | バイナリ（=本フェーズ）では `anyhow` を使う |
| `thiserror` | エラー型の定義 | ライブラリ層向け（次フェーズ以降で本格活用） |

### `anyhow` と `thiserror` の使い分け

- **アプリ（バイナリ）**: `anyhow::Result<T>` で気軽にエラーを扱う
- **ライブラリ**: `thiserror` でエラー型を定義し、呼び出し側に意味のあるエラーを返す
- 本フェーズはCLI（バイナリ）なので **`anyhow` をメイン** に使う

## 学習ポイント

- 構造体・列挙型の実戦的な設計
- ファイルI/O（読み書き）
- エラー処理の実用パターン
- 外部クレートの使い方（`Cargo.toml` の編集）
- モジュール分割での設計
- ユニットテスト + 統合テスト（`assert_cmd` + `predicates`）
- `cargo fmt` / `cargo clippy` による品質維持
- ドキュメンテーションコメント（`///`）と `cargo doc`
