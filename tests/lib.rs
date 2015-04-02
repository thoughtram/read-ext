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
