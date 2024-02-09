#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MyTrait {
    fn foo(&self);
}

pub struct MyStruct {}

impl MyTrait for MyStruct {
    fn foo(&self) {}
}
