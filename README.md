Recording server to work alongside an [Odoo's sfu](https://github.com/odoo/sfu).

## Requirements

* [FFMPEG](https://www.ffmpeg.org/)
* [Rust](https://www.rust-lang.org/) v1.80 or later

## Installation

# compiling flatbuffer schema

```bash
flatc --rust -o src/misc flatbuffers/schema.fbs 
```

# compiling the server

```bash
cargo build --release
```
