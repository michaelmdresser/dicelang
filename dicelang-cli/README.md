# CLI dicelang

Roll dice in your shell!

``` sh
cargo build
alias roll="$(pwd)/target/debug/dicelang-cli"
```

``` sh
→ roll d20+1d4-1d6+3
d20:11 d4:2 d6:4
1d20+1d4-1d6+3 => 12
```
