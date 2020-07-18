# Crossclip

[![Travis Build Status](https://travis-ci.org/mcobzarenco/crossclip.svg?branch=master)](https://travis-ci.org/mcobzarenco/crossclip)
[![](https://img.shields.io/crates/v/crossclip.svg)](https://crates.io/crates/crossclip)
[![](https://docs.rs/crossclip/badge.svg)](https://docs.rs/crossclip)

Crossclip is a cross-platform Rust library for interfacing with the system
clipboard. It has been tested on GNU/Linux, FreeBSD, Windows and Mac OSX.

The library is a fork of [clipboard2](), itself a fork of [rust-clipboard](https://github.com/aweinstock314/rust-clipboard)

## Prerequisites

On Linux you need the x11 library. On a Debian-y distribution, install it with something like

```bash
sudo apt install xorg-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

## Example

```rust
use crossclip::{Clipboard, SystemClipboard, Result};

fn main() -> Result<()> {
    let clipboard = SystemClipboard::new()?;
    clipboard.set_string_contents(String::from("Hello"))?;
    println!("{}", clipboard.get_string_contents()?);
}
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.
