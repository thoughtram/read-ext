#Extensions for Rusts standard `Read` trait

This adds `read_into_string` and `read_into_vec` as specified in this [RFC](https://github.com/cburgdorf/rfcs/blob/read_to_string/text/0841-read_to_string_without_buffer.md).

##How to use

```Rust
extern crate readext;

use std::io::{Cursor};
use readext::ReadExt;

#[test]
fn can_read_into_string () {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes as &[u8]);
    let s = input.read_into_string().unwrap();
    assert_eq!(s, "hello");
}


#[test]
fn can_read_into_vec () {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes as &[u8]);
    let s = input.read_into_vec().unwrap();
    assert_eq!(s.len(), 5);
}
```

