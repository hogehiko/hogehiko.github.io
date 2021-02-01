---
layout: post
title:  自作言語のLanguage Serverを作る
date:   2021-01-30 00:30:00 +0900
categories: programming lsp
---

プログラミング言語は強力なエディタ・IDEのサポートがあってはじめて実力を発揮します。
自作のプログラミング言語にエディタサポートを付けるため、Language Protocol の Client/Server を VSCode プラグインとして実装してみます。

# Language Server Protocol (LSP)

Language Serverはクライアント（エディタ等）からの問い合わせに対し、自動保管やコードジャンプの情報を返すサーバです。
対応する機能の一部を列挙しましょう。

* completion
* definition
* rename
* formatting
* 他多数

普段使う機能は一通り揃っているようです。

通信はJSON-RPCで実現されます。プロトコルの詳細については本稿では扱いません。


# LSP サンプルを実行

microsoftのvscode-extension-samples リポジトリにある lsp-sample は、clone してvscodeを開くだけで Language Server を含む VSCode言語サポートの開発を始めることができます。

サンプルで実装されている機能を見てみましょう。

## 自動補完


## エラー表示


# 自作言語の組み込み

## 変数つき電卓

自作の電卓を組み込みます。

## オートコンプリートを実装

## エラー表示を実装

a