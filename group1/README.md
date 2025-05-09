# group1

You can use [RustOwl](https://github.com/cordx56/rustowl), rust-analyzer, and rustc.

---

RustOwlは以下の色の下線を用いて可視化を行う。

- 緑：変数の実際のライフタイムの範囲
- 青：不変借用が行われている位置
- 紫：可変借用が行われている位置
- 橙：所有権の移動および関数呼び出し
- 赤：エラー
  - ライフタイムが不正となっている範囲
  - 可変借用と他の借用が共存している位置
