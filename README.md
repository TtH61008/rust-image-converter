# image_converter on rust

pythonで書いたimage_converterをChatGPTにRustで書き直させてみた


## Usage

```bash
bin/image_converter --input-dir ./trial/color --output-dir ./trial/gray
```


## Build

```bash
cargo build --release
./target/release/image_converter --input-dir ./trial/color --output-dir ./trial/gray 
```

