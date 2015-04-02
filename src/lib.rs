use std::io::{ Result, Read };

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
}

impl<T> ReadExt for T where T: Read {}
