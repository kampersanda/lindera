```
cargo run --release -p lindera-cli --features=ipadic
```

All wakachi-gaki

```
cargo run --release -p lindera-cli --features=ipadic -- -O wakati < wagahaiwa_nekodearu.txt > tokenized.ipadic.txt
cargo run --release -p lindera-cli --features=unidic -- -O wakati < wagahaiwa_nekodearu.txt > tokenized.unidic.txt
```

Timeperf

```
cargo run --release -p timeperf --features=ipadic -- -s wagahaiwa_nekodearu.txt -k ipadic
cargo run --release -p timeperf --features=unidic -- -s wagahaiwa_nekodearu.txt -k unidic
```