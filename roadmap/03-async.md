# フェーズ3: 非同期処理（async/await）

## ゴール

Rustの非同期処理の仕組みと、`tokio` ランタイムの使い方を理解する。Web API への前準備。

## 学習トピック

### 1. 同期と非同期の違い
- ブロッキングとノンブロッキング
- なぜ非同期が必要か（I/O待ちの効率化）
- **Pythonとの比較メモ**: Python の `asyncio` の経験が活かせる

### 2. Future
- `Future` トレイトとは
- `poll` の概念
- `.await` の意味

### 3. async / await
- `async fn` の定義
- 戻り値が `Future` になる仕組み
- `.await` で値を取り出す

### 4. tokio ランタイム
- ランタイムとは
- `#[tokio::main]` マクロ
- `tokio::spawn` でタスクを並行実行
- `tokio::join!` で複数を待つ

### 5. 並行と並列
- 並行（concurrency）と並列（parallelism）の違い
- async は並行、スレッドは並列

### 6. async 環境のエラー処理
- `Result` を返す async 関数
- `?` 演算子は async でも使える

### 7. チャネル（`tokio::sync::mpsc`）
- 非同期タスク間の通信
- `tokio::sync::mpsc::channel` の使い方
- 送信側 / 受信側の分離

### 8. 共有状態
- `Arc<Mutex<T>>` の基本
- **重要な落とし穴**: `std::sync::Mutex` を `.await` をまたいで保持しないこと
- async 文脈では `tokio::sync::Mutex` / `tokio::sync::RwLock` を使う場面がある
- 使い分け: ロック保持時間が短く `.await` を含まない → `std::sync::Mutex` でOK

### 9. 複数 Future の選択（`tokio::select!`）
- 複数の Future の中で「最初に完了したもの」を選ぶ
- キャンセル安全性（cancel safety）の概念に軽く触れる

### 10. Stream トレイト
- `Future` の複数値版（イテレータの非同期版のイメージ）
- sqlx の結果ストリームなどで触れる

### 11. よくある落とし穴
- `Send` トレイト境界
- ライフタイムと async
- ブロッキング関数を async で呼ぶ問題（`tokio::task::spawn_blocking`）

## このフェーズで作るもの

軽い練習用コードのみ：
- 複数のHTTPリクエストを並行実行する練習（`reqwest` 使用）
- `tokio::time::sleep` を使った遅延処理
- mpsc チャネルでタスク間通信
- `tokio::select!` で複数 Future の選択
