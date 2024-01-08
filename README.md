
# Using `ByteDance` mirroring service

```bash
cd ~/.cargo
```

In conf file, add:

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```

>| Reference:
https://rsproxy.cn/


# Build

```bash
cargo build --release
```

# Run

```bash
./target/release/leetcode-rs
```
