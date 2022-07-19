```
cargo run --release -p lindera-cli --features=ipadic
```

All wakachi-gaki

```
cargo run --release -p lindera-cli --features=ipadic -- -O wakati < wagahaiwa_nekodearu.txt > tokenized.ipadic.txt
cargo run --release -p lindera-cli --features=unidic -- -O wakati < wagahaiwa_nekodearu.txt > tokenized.unidic.txt
```