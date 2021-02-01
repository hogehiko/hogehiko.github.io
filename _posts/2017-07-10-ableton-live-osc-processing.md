---
layout: post
title:  "OSCでAbleton LiveとProcessingを接続する"
date:   2017-07-10 15:19:21 +0900
categories: processing abletonlive
---

Max for Live向けの「[Connection Kit]」という面白いPackが配布されています。
これは、LiveのMIDIデータを外部のシステムと接続するためのツールで、現時点ではArduino, Lego Mindstormといったデバイスとの接続が可能です。

Connection Kitは、OSC(Open Sound Control)と呼ばれる音楽制御情報送受信のためのプロトコルもサポートしていますので、OSCを使ってAbleton LiveとProcessingを接続してみます。

# Live側のセットアップ

今回はConnection Kitの中でもMIDI信号をそのまま送信する"OSC Midi Send(Max MIDI Effect)"を使います。任意のMIDIチャンネルにOSC Midi Sendを挿入します。

![OSC Midi Send]({{ site.url }}/images/live9_osc_midi_send.png)

HostとPortを指定する欄があります。ここではこのまま使用します。
このままキーボード等を操作すると、NoteとVelocityの値が表示されるかと思います。

これでLive側の準備は完了。

# Processing側の実装

次にProcessing側でこの信号を受けましょう。

まずは[oscP5]というライブラリを追加します。
ツール→ツールを追加　で追加できます。

![oscP5を追加]({{ site.url }}/images/processing_lib.png)

受信した信号を標準出力に表示してみましょう。

```java
import oscP5.*;

OscP5 oscP5;

void setup(){
  oscP5 = new OscP5(this, 2346);
}

void oscEvent(OscMessage msg){
  println(msg);
}
```

この状態でLiveでキーボード操作等をすると、以下のようなメッセージが表示されるかと思います。

```
/127.0.0.1:60267 | /Note1 i
/127.0.0.1:60267 | /Velocity1 i
```

この、`/Note1`や`/Velocity1`といった文字列はパスのようなものです。どの程度規格化されているのか筆者は調べていませんが、少なくともOSC Midi Sendはノートごとに`/Note[連番]=音の高さ`, `/Velocity[連番]=音の強さ`という値を送るようです。
また、和音を演奏すると連番の値が増えていくようです。

右端の`i`はパラメータの型を示しているようで、iは整数（integer)を意味しているようです。

コードを修正してVelocityを取り出してみます。

```java
import oscP5.*;

OscP5 oscP5;

void setup(){
  oscP5 = new OscP5(this, 2346);
}

void oscEvent(OscMessage msg){
  if(msg.checkAddrPattern("/Velocity1")){
    println(msg.get(0).intValue());  
  }
}
```

演奏すると以下のようなメッセージが表示されます。

```
115
0
122
0
```

0はキーをリリースしたタイミングで出力されます。MIDIのVelocityなので最大値は127ぽいです。

これをどのように料理するかはアイデア次第ですが、Velocityによって画面をフラッシュさせるものを作成してみました。

```java
import oscP5.*;

OscP5 oscP5;

int bgBrightness = 0;

void setup(){
  oscP5 = new OscP5(this, 2346);
}

void draw(){
  background(bgBrightness);
  if(bgBrightness > 0){
    bgBrightness --;
  }
}

void oscEvent(OscMessage msg){
  if(msg.checkAddrPattern("/Velocity1")){
    int velocity = msg.get(0).intValue();
    int newBrightness = velocity * 2;
    if(bgBrightness < newBrightness){
      bgBrightness = newBrightness;
    }
  }
}
```

(キャプチャは割愛します)

以上です。

[Connection Kit]: https://www.ableton.com/ja/packs/connection-kit/
[oscP5]: http://www.sojamo.de/libraries/oscP5