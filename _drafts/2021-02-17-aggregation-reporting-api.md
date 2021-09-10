---
layout: post
title:  
date:   2021-02-17 00:30:00 +0900
categories: browser adtech
---

# Aggregate Reporting APIメモ

## 概要


https://github.com/WICG/conversion-measurement-api/blob/master/SERVICE.md
https://github.com/WICG/conversion-measurement-api/blob/master/AGGREGATE.md
https://github.com/csharrison/aggregate-reporting-api

## 広告のリーチを計測

* event-level
* aggregation

event-levelだけではカバーできない情報を匿名化した集約情報として扱う仕組み。

集計データを a タグに追加
aggimpressiondata=”a=xyz,b=20”

コンバージョンの場合
agg-conversion-data=”type=purchase&conversion-value=123


* ブラウザ
* Report Origin
* Helper

拡張によって
vtcv
mta
直服CV