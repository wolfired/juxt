juxt_basex
================

[![Crates.io Version](https://img.shields.io/crates/v/juxt_basex?style=flat)](https://crates.io/crates/juxt_basex)
[![docs.rs](https://img.shields.io/docsrs/juxt_basex?style=flat&logo=docsdotrs)](https://docs.rs/juxt_basex/latest/juxt_basex/)
<!-- [![Codecov](https://img.shields.io/codecov/c/gh/wolfired/juxt?token=95IHYGJI9H&style=flat&logo=codecov)](https://app.codecov.io/gh/wolfired/juxt) -->

just basex, nothing else

# Usage

```rust

use std::fs::read;
use std::str::FromStr;

use juxt_basex::Base64;

fn main() {
    println!("{}", Base64::from_str("juxt_basex").unwrap());
}

```

# External Reference

[base16/base32/base64/rfc4648](https://www.ietf.org/rfc/rfc4648.txt)

