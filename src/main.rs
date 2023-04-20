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
