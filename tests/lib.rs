extern crate readext;

use readext::ReadExt;
use std::io::Cursor;

#[test]
fn can_read_into_string() {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes as &[u8]);
    let s = input.read_into_string().unwrap();
    assert_eq!(s, "hello");
}

#[test]
fn can_read_into_vec() {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes as &[u8]);
    let s = input.read_into_vec().unwrap();
    assert_eq!(s.len(), 5);
}

#[test]
fn can_read_into_array_exact() {
    let bytes = b"hello";
    let mut input = Cursor::new(bytes as &[u8]);
    let s = input.read_into_array_exact::<5>().unwrap();
    assert_eq!(s, [b'h', b'e', b'l', b'l', b'o']);
    let s = input.read_into_array_exact::<4>();
    assert!(s.is_err());
    let s = input.read_into_array_exact::<6>();
    assert!(s.is_err());
}
