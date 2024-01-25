juxt_adler32
================

[![Crates.io Version](https://img.shields.io/crates/v/juxt_adler32?style=flat)](https://crates.io/crates/juxt_adler32)
[![docs.rs](https://img.shields.io/docsrs/juxt_adler32?style=flat&logo=docsdotrs)](https://docs.rs/juxt_adler32/latest/juxt_adler32/)
<!-- [![Codecov](https://img.shields.io/codecov/c/gh/wolfired/juxt?token=95IHYGJI9H&style=flat&logo=codecov)](https://app.codecov.io/gh/wolfired/juxt) -->

just adler32, nothing else

# Usage

```rust

use std::str::FromStr;

use juxt_adler32::Adler32;

fn main() {
    println!("{}", Adler32::from_str("juxt_adler32").unwrap());
}

```

# External Reference

[adler32/rfc1950](https://www.ietf.org/rfc/rfc1950.txt)
