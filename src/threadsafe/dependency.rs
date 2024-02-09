use dyn_clone::{clone_trait_object, DynClone};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MyTrait: DynClone + Send + Sync {
    fn foo(&self);
}
clone_trait_object!(MyTrait);

#[derive(Clone)]
pub struct MyStruct {}

impl MyTrait for MyStruct {
    fn foo(&self) {}
}

#[cfg(test)]
mod test {
    use crate::dependency::MockMyTrait;

    impl Clone for MockMyTrait {
        fn clone(&self) -> Self {
            panic!("Clone was attempted for mock!")
        }

        fn clone_from(&mut self, _: &Self) {
            panic!("clone_from was attempted for mock!")
        }
    }
}
