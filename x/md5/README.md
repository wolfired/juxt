juxt_md5
================

[![Crates.io Version](https://img.shields.io/crates/v/juxt_md5?style=flat)](https://crates.io/crates/juxt_md5)
[![docs.rs](https://img.shields.io/docsrs/juxt_md5?style=flat&logo=docsdotrs)](https://docs.rs/juxt_md5/latest/juxt_md5/)
<!-- [![Codecov](https://img.shields.io/codecov/c/gh/wolfired/juxt?token=95IHYGJI9H&style=flat&logo=codecov)](https://app.codecov.io/gh/wolfired/juxt) -->

just md5, nothing else

# Usage

```rust

use std::fs::read;
use std::str::FromStr;

use juxt_md5::Md5;

fn main() {
    println!("{}", Md5::from_str("juxt_md5").unwrap());
    println!("{}", Md5::from_vec(&mut read("path to a file").unwrap()));
}

```

# External Reference

[md5/rfc1321](https://www.ietf.org/rfc/rfc1321.txt)
