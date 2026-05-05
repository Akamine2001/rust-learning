# rust-learning

Rust言語を習得するための学習リポジトリ。

## 概要

PythonユーザーがRustを段階的に学んでいく過程の記録。
最終的にタスク管理 Web API（axum + SQLite + JWT認証）を作ることを目標とする。

## 構成

```
rust-learning/
├── CLAUDE.md                 # Claude（学習サポート）の振る舞いルール
├── roadmap/                  # 学習ロードマップとTODO・メモ
│   ├── README.md
│   ├── 01-basics.md          # フェーズ1: 基礎構文
│   ├── 01-basics-todo.md
│   ├── 01-basics-notes.md
│   ├── 02-cli.md             # フェーズ2: CLIプロジェクト
│   ├── 02-cli-todo.md
│   ├── 02-cli-notes.md
│   ├── 03-async.md           # フェーズ3: 非同期処理
│   ├── 03-async-todo.md
│   ├── 03-async-notes.md
│   ├── 04-web-api.md         # フェーズ4: Web API
│   ├── 04-web-api-todo.md
│   └── 04-web-api-notes.md
├── practice/                 # Rustプロジェクト・練習コードの置き場
│   └── README.md             # 配置ルール
└── .claude/skills/           # 学習サポート用 skill
    ├── learning-status/      # 進捗確認
    ├── learning-memo/        # 学習メモ自動記録
    └── daily-summary/        # 1日のまとめとpush
```

学習中に作るRustプロジェクトはすべて `practice/` 配下に配置する（詳細は `practice/README.md`）。

## 学習フェーズ

| フェーズ | 内容 | 成果物 |
|---|---|---|
| 1 | 基礎構文 | - |
| 2 | CLI プロジェクト | タスク管理CLI |
| 3 | 非同期処理 | - |
| 4 | Web API プロジェクト | タスク管理Web API（JWT認証付き） |

## 学習方針

Claude とペアで学習を進める。詳細は `CLAUDE.md` 参照。
