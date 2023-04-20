use std::cell::RefCell;
use std::rc::Rc;

trait NotMyCodeTrait {
    fn foo(&self);
}

struct NotMyCodeStruct {}

impl NotMyCodeTrait for NotMyCodeStruct {
    fn foo(&self) {}
}

fn not_my_builder_for_not_my_code() -> Box<dyn NotMyCodeTrait> {
    Box::new(NotMyCodeStruct {})
}

struct ThisIsMyStruct {
    dependency: Rc<RefCell<Box<dyn NotMyCodeTrait>>>,
}

impl ThisIsMyStruct {
    fn new() -> Self {
        Self {
            dependency: Rc::new(RefCell::new(not_my_builder_for_not_my_code())),
        }
    }

    fn do_foo(&self) {
        self.dependency.borrow().foo();
    }
}

fn main() {
    let my_struct = ThisIsMyStruct::new();
    my_struct.do_foo();
    println!("The foo is done!");
}

#[cfg(test)]
mod test {
    use mockall::mock;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{NotMyCodeTrait, ThisIsMyStruct};

    mock! {
        MyNotMyCodeTrait {}

        impl NotMyCodeTrait for MyNotMyCodeTrait {
            fn foo(&self);
        }
    }

    struct TestContext {
        mock_port: Rc<RefCell<Box<MockMyNotMyCodeTrait>>>,
        testable: ThisIsMyStruct,
    }

    impl TestContext {
        fn new() -> Self {
            let mocked_dependency = Rc::new(RefCell::new(Box::new(MockMyNotMyCodeTrait::new())));
            Self {
                mock_port: Rc::clone(&mocked_dependency),
                testable: ThisIsMyStruct {
                    dependency: mocked_dependency,
                },
            }
        }
    }
}
