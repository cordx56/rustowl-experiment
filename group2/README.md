# group2

You can use [Aquascope](https://cel.cs.brown.edu/aquascope/), rust-analyzer, and rustc.

---

Aquascopeは実行時のメモリの可視化（Interpret）と所有権の可視化（Permissions）ができるツールです。

メモリの可視化は、n行目に`Ln`という表示が割り当てられ、その時のスタック領域とヒープ領域の変数名および値が見られます。
所有権の可視化は、読み込みができる場合にはR、書き込みができる場合にはW、所有権を持っている場合にはOという表示が、各変数に表示されます。
