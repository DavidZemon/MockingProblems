use std::cell::RefCell;
use std::rc::Rc;

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
    port: Rc<RefCell<Box<dyn serialport::SerialPort>>>,
}

impl MyStruct {
    fn new() -> Self {
        Self {
            port: Rc::new(RefCell::new(serialport::new())),
        }
    }

    fn do_send(&self) {
        self.port.borrow().send();
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::serialport;
    use crate::MyStruct;

    mock! {
        SerialPort {}

        impl serialport::SerialPort for SerialPort {
            fn send(&self);
        }
    }

    struct TestContext {
        mock_port: Rc<RefCell<Box<MockSerialPort>>>,
        testable: MyStruct,
    }

    impl TestContext {
        fn new() -> Self {
            let mock_port = Rc::new(RefCell::new(Box::new(MockSerialPort::new())));
            Self {
                mock_port: Rc::clone(&mock_port),
                testable: MyStruct { port: mock_port },
            }
        }
    }
}
