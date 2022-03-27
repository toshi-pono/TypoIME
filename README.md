# TypoIME

typoIME は、英数字入力を支援する macOS 向け IME です。  
アルファベットがこっそり変換されるので、英数字入力を楽しむことができます。  

現在、アルファベット1文字の変換にのみ対応しています。

※ 注意：ネタIMEです

## 動作例

original
```
hello world!
HELLO WORLD!
```

output
```
heI1o　worId!
HELLO W0RLD! 
```

## 動作環境

以下の環境で動作を確認しています  
macOS (Intel) 11.5 Big Sur  

## 使い方

### 1. ビルド ・ パッケージの出力

```sh
$ make app
```

`output/typoIME.app` が出力されます

### 2. Install

typoIME.app を `/Library/Input Methods` 内にコピーします。  
再起動（または、ログアウト→再ログイン）することで有効になります。

環境設定 > キーボード > 入力ソース の英語から`TypoIME`を追加してください。
