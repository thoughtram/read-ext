
extern crate readext;

use std::io::{Read,Cursor};
use readext::ReadExt;

#[test]
fn test () {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes);
    let s = input.read_into_string();
    assert_eq!(s, "hello");
}
