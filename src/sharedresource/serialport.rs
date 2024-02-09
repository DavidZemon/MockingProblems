use std::io;

pub trait SerialPort: Send + io::Read {
    fn send(&self);
}

pub struct SomeConcreteSerialPort {}

impl SerialPort for SomeConcreteSerialPort {
    fn send(&self) {}
}

impl io::Read for SomeConcreteSerialPort {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

pub fn new() -> Box<dyn SerialPort> {
    Box::new(SomeConcreteSerialPort {})
}
