# solvability_checker

## 概要 / Overview
- ソルバー群を一定間隔で並列実行し、失敗が続いたら Webhook に通知するシンプルな監視ツール。
- Runs all solver scripts periodically in parallel; if a solver keeps failing, it posts a JSON payload to a webhook.

## インストール / Install
```bash
cargo install solvability_checker
```

## 使い方 / Usage
ソルバをディレクトリに置いて実行権限を付けておく。終了ステータスで成否を判断できるようにする。
```bash
solvability_checker \
  --url https://example.com/webhook \   # required
  --solver solver \                     # default: solver
  --interval 3000 \                     # ms, default: 3000
  --retries 5                           # default: 5
```

### オプション / Options
- `--url`, `-u`: Webhook URL (必須 / required)
- `--solver`, `-s`: ソルバーディレクトリ (default: `solver`)
- `--interval`, `-i`: 周回間隔ミリ秒 (default: `300,000`)
- `--retries`, `-r`: 失敗時リトライ回数 (default: `5`)

### ログ / Logs
`env_logger` を利用。例: `RUST_LOG=info solvability_checker ...`
