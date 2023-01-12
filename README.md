# css2slint

Silly attempt at crudely extracting CSS variables from a .css/.scss file and converting it into Slint code, that can be massaged further or edited into "shape".

## Prerequisites

You need to [install Rust](https://www.rust-lang.org/tools/install) before you can run this.

## Running

```sh
cargo run /path/to/foo.scss > foo.slint
```

