# Dioxus Markdown Blog

Rust と Dioxus 0.7 で構築された静的ブログアプリケーションです。

## 機能

- **Markdown レンダリング**: GFM（GitHub Flavored Markdown）対応（テーブル、取り消し線、タスクリスト）
- **XSS 対策**: ammonia による HTML サニタイズ
- **外部リンク**: 自動的に新しいタブで開く
- **レスポンシブデザイン**: Tailwind CSS によるモバイル対応レイアウト
- **ダークモード**: システム設定に自動追従
- **型安全なルーティング**: Dioxus Router によるコンパイル時チェック
- **YAML フロントマター**: 記事メタデータの管理
- **記事の自動検出**: `content/posts/` 配下の Markdown を自動で読み込み
- **CI/CD**: GitHub Actions による自動テスト・ビルド

## プロジェクト構成

```
dioxus-playground/
├── Cargo.toml              # ワークスペースマニフェスト
├── Dioxus.toml             # Dioxus CLI 設定
├── package.json            # Bun/npm スクリプト
├── apps/
│   └── web/                # Web アプリケーション
│       ├── assets/
│       │   ├── input.css   # Tailwind 入力ファイル
│       │   └── main.css    # 生成された CSS（gitignore）
│       └── src/
│           ├── main.rs     # エントリーポイント・ルーティング
│           ├── components/ # 再利用可能なコンポーネント
│           └── pages/      # ページコンポーネント
├── crates/
│   ├── content/            # 記事データ構造・ローダー
│   │   └── build.rs        # 記事の自動検出
│   └── markdown/           # Markdown パーサー・サニタイザー
├── content/
│   └── posts/              # Markdown 記事ファイル
│       └── *.md
└── .github/
    └── workflows/
        └── ci.yml          # GitHub Actions CI
```

## 必要要件

- Rust 1.70 以上
- Dioxus CLI
- Bun（または Node.js）

## セットアップ

### 1. Dioxus CLI のインストール

```bash
cargo install dioxus-cli
```

### 2. 依存関係のインストール

```bash
bun install
```

### 3. CSS のビルド

```bash
bun run css
```

### 4. 開発サーバーの起動

```bash
bun run dev
```

または個別に起動する場合:

```bash
# ターミナル 1: CSS のウォッチ
bun run css:watch

# ターミナル 2: Dioxus 開発サーバー
dx serve --platform web --package web --port 8081
```

ブラウザで http://localhost:8081 を開きます。

## ビルド

本番用ビルドを作成するには:

```bash
bun run build
```

または個別に:

```bash
bun run css
dx build --platform web --package web --release
```

ビルド成果物は `dist/` ディレクトリに出力されます。

## テスト

```bash
cargo test --workspace
```

## Lint・フォーマット

```bash
# フォーマット
cargo fmt --all

# Lint チェック
cargo clippy --workspace -- -D warnings
```

## ルーティング

| パス | ページ | 説明 |
|------|--------|------|
| `/` | Home | 記事一覧 |
| `/posts/:slug` | Post | 記事詳細 |
| `/about` | About | サイト情報 |
| `/*` | NotFound | 404 ページ |

## 記事の追加

`content/posts/` ディレクトリに Markdown ファイルを追加します。

### フロントマター形式

```yaml
---
title: 記事タイトル
date: 2024-01-15
description: 記事の説明文
tags:
  - rust
  - dioxus
draft: false  # true にすると非公開
---

# 本文

Markdown で記事を書きます。
```

### 対応している Markdown 記法

- 見出し（h1〜h6）
- 段落・改行
- **太字**、*斜体*、~~取り消し線~~
- リンク、画像
- 順序付き・順序なしリスト
- コードブロック（言語指定可）
- テーブル
- 引用
- 水平線

## 技術スタック

| クレート / ツール | バージョン | 用途 |
|-------------------|------------|------|
| dioxus | 0.7 | Web フレームワーク |
| pulldown-cmark | 0.10 | Markdown パース |
| ammonia | 4.0 | HTML サニタイズ |
| yaml-front-matter | 0.1 | フロントマター解析 |
| chrono | 0.4 | 日付処理 |
| serde | 1.0 | シリアライズ |
| Tailwind CSS | 4.0 | スタイリング |
| Bun | - | パッケージマネージャー |

## CI/CD

GitHub Actions で以下を自動実行:

- **Format**: `cargo fmt` によるコードフォーマットチェック
- **Clippy**: `cargo clippy` による静的解析
- **Check**: `cargo check` によるコンパイルチェック
- **Test**: `cargo test` によるテスト実行
- **Build**: Web アプリケーションのビルド

## ライセンス

MIT
