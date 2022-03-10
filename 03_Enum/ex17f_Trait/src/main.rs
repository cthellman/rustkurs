/*  ex17f_Trait  */

// https://doc.rust-lang.org/std/io/trait.Write.html

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;   // low level write, full control
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;  // better use this in most cases
    ...


