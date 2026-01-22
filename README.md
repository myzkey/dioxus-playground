# Dioxus Markdown Blog

Rust と Dioxus 0.6 で構築された静的ブログアプリケーションです。

## 機能

- **Markdown レンダリング**: GFM（GitHub Flavored Markdown）対応（テーブル、取り消し線、タスクリスト）
- **XSS 対策**: ammonia による HTML サニタイズ
- **外部リンク**: 自動的に新しいタブで開く
- **レスポンシブデザイン**: モバイル対応レイアウト
- **ダークモード**: システム設定に自動追従
- **型安全なルーティング**: Dioxus Router によるコンパイル時チェック
- **YAML フロントマター**: 記事メタデータの管理

## プロジェクト構成

```
dioxus-playground/
├── Cargo.toml              # ワークスペースマニフェスト
├── Dioxus.toml             # Dioxus CLI 設定
├── apps/
│   └── web/                # Web アプリケーション
│       ├── assets/
│       │   └── main.css    # スタイルシート
│       └── src/
│           ├── main.rs     # エントリーポイント・ルーティング
│           ├── components/ # 再利用可能なコンポーネント
│           └── pages/      # ページコンポーネント
├── crates/
│   ├── content/            # 記事データ構造・ローダー
│   └── markdown/           # Markdown パーサー・サニタイザー
└── content/
    └── posts/              # Markdown 記事ファイル
        └── hello-world.md
```

## 必要要件

- Rust 1.70 以上
- Dioxus CLI

## セットアップ

### 1. Dioxus CLI のインストール

```bash
cargo install dioxus-cli
```

### 2. 依存関係のインストール

```bash
cargo build
```

### 3. 開発サーバーの起動

```bash
dx serve --platform web
```

ブラウザで http://localhost:8080 を開きます。

## ビルド

本番用ビルドを作成するには:

```bash
dx build --platform web --release
```

ビルド成果物は `dist/` ディレクトリに出力されます。

## テスト

```bash
cargo test --workspace
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

| クレート | バージョン | 用途 |
|----------|------------|------|
| dioxus | 0.6 | Web フレームワーク |
| pulldown-cmark | 0.10 | Markdown パース |
| ammonia | 4.0 | HTML サニタイズ |
| yaml-front-matter | 0.1 | フロントマター解析 |
| chrono | 0.4 | 日付処理 |
| serde | 1.0 | シリアライズ |

## ライセンス

MIT
