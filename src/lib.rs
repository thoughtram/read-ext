use std::io::{Read, Result};

pub trait ReadExt: Read {
    /// Read all bytes until EOF in this source, returning them as a new `Vec`.
    ///
    /// See `read_to_end` for other semantics.
    fn read_into_vec(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let res = self.read_to_end(&mut buf);
        res.map(|_| buf)
    }

    /// Read all bytes until EOF in this source, returning them as a new buffer.
    ///
    /// See `read_to_string` for other semantics.
    fn read_into_string(&mut self) -> Result<String> {
        let mut buf = String::new();
        let res = self.read_to_string(&mut buf);
        res.map(|_| buf)
    }

    /// Reads a specific amount of bytes into an array, returning it as a new buffer.
    ///
    /// See `Read::read_exact` for other semantics.
    fn read_into_array_exact<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]> {
        let mut buffer = [0; SIZE];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl<T> ReadExt for T where T: Read {}
