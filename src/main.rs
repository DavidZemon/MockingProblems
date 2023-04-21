/// This is not my code. This is a simplified version of the serialport crate.
mod serialport {
    pub trait SerialPort {
        fn send(&self);
    }

    pub struct SomeConcreteSerialPort {}

    impl SerialPort for SomeConcreteSerialPort {
        fn send(&self) {}
    }

    pub fn new() -> Box<dyn SerialPort> {
        Box::new(SomeConcreteSerialPort {})
    }
}

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
    use std::rc::Rc;

    use crate::serialport;
    use crate::MyStruct;

    mock! {
        SerialPort {}

        impl serialport::SerialPort for SerialPort {
            fn send(&self);
        }
    }

    struct SharedMockSerialPort(Rc<MockSerialPort>);
    impl serialport::SerialPort for SharedMockSerialPort {
        fn send(&self) {
            self.0.send();
        }
    }

    struct TestContext {
        mock_port: Rc<MockSerialPort>,
        testable: MyStruct,
    }

    impl TestContext {
        fn new() -> Self {
            let mock_port = Rc::new(MockSerialPort::new());
            Self {
                mock_port: Rc::clone(&mock_port),
                testable: MyStruct {
                    port: Box::new(SharedMockSerialPort(mock_port)),
                },
            }
        }
    }

    #[test]
    fn test_happy_path() {
        let context = TestContext::new();

        context.mock_port.expect_send().once();

        context.testable.do_send();
    }
}
