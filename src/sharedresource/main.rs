mod serialport;

struct MyStruct {
    port: Box<dyn serialport::SerialPort>,
}

impl MyStruct {
    fn new() -> Self {
        Self {
            port: serialport::new(),
        }
    }

    fn do_send(&self) {
        self.port.send();
    }
}

fn main() {
    let my_struct = MyStruct::new();
    my_struct.do_send();
    println!("The foo is done!");
}

#[cfg(test)]
mod test {
    use mockall::mock;
    use std::io;
    use std::sync::{Arc, Mutex};

    use crate::serialport;
    use crate::MyStruct;

    mock! {
        SerialPort {}

        impl serialport::SerialPort for SerialPort {
            fn send(&self);
        }

        impl io::Read for SerialPort {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
        }
    }

    struct SharedMockSerialPort(Arc<Mutex<MockSerialPort>>);
    impl serialport::SerialPort for SharedMockSerialPort {
        fn send(&self) {
            self.0.lock().unwrap().send()
        }
    }
    impl io::Read for SharedMockSerialPort {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.0.lock().unwrap().read(buf)
        }
    }

    struct TestContext {
        mock_port: Arc<Mutex<MockSerialPort>>,
        testable: MyStruct,
    }

    impl TestContext {
        fn new() -> Self {
            let mock_port = Arc::new(Mutex::new(MockSerialPort::new()));
            Self {
                mock_port: Arc::clone(&mock_port),
                testable: MyStruct {
                    port: Box::new(SharedMockSerialPort(mock_port)),
                },
            }
        }
    }

    #[test]
    fn test_happy_path() {
        let context = TestContext::new();

        context
            .mock_port
            .lock()
            .unwrap()
            .expect_send()
            .once()
            .return_const(());

        context.testable.do_send();
    }
}
