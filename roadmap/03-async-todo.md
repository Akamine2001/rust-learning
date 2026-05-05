# フェーズ3: 非同期処理 TODO

## 学習
- [ ] 同期 vs 非同期の違いを理解
- [ ] Future トレイトの概念を理解
- [ ] `async fn` と `.await` の文法を試す
- [ ] `#[tokio::main]` でランタイム起動

## 実習: 基本
- [ ] `tokio::time::sleep` で遅延を作る
- [ ] `tokio::spawn` でタスクを並行起動
- [ ] `tokio::join!` で複数 await を同時実行
- [ ] `reqwest` で HTTP GET を非同期で叩く
- [ ] 複数HTTPリクエストを並行実行する例を書く

## 実習: チャネル
- [ ] `tokio::sync::mpsc::channel` で送信・受信を試す
- [ ] producer / consumer パターンを書いてみる

## 実習: 共有状態
- [ ] `Arc<Mutex<T>>` で複数タスクから共有データにアクセス
- [ ] `std::sync::Mutex` を `.await` をまたいで保持して怒られる例を書く
- [ ] `tokio::sync::Mutex` に置き換えて解決する

## 実習: select!
- [ ] `tokio::select!` で複数 Future の最初の完了を待つ
- [ ] キャンセル安全性の概念を学ぶ

## 実習: Stream
- [ ] `Stream` トレイトの概念に触れる（深掘りはしない）

## エラー処理
- [ ] `Result` を返す async 関数を書く
- [ ] `?` 演算子で async 内のエラーを伝播

## 落とし穴の理解
- [ ] `Send` 境界エラーを実際に出してみる
- [ ] ブロッキング関数を async で呼ぶとどうなるかを試す（`tokio::task::spawn_blocking`）

## フェーズ完了チェック
- [ ] 非同期コードを書ける
- [ ] フェーズ4に進む準備完了
