use crate::dependency::MyTrait;
use dyn_clone::{clone_trait_object, DynClone};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Middleman: DynClone + Send + Sync {
    fn do_middling_things(&self);
}
clone_trait_object!(Middleman);

#[derive(Clone)]
pub struct ConcreteMiddleman {
    my_trait: Box<dyn MyTrait>,
}

impl ConcreteMiddleman {
    /// This is some asynchronous operation, and it takes time to build the other properties of a
    /// ConcreteMiddleman instance
    pub async fn new(my_trait: Box<dyn MyTrait>) -> Self {
        Self { my_trait }
    }
}

impl Middleman for ConcreteMiddleman {
    fn do_middling_things(&self) {
        self.my_trait.foo()
    }
}

#[cfg(test)]
mod test {
    use crate::dependency::MockMyTrait;
    use crate::middleman::{ConcreteMiddleman, Middleman, MockMiddleman};

    impl Clone for MockMiddleman {
        fn clone(&self) -> Self {
            panic!("Clone was attempted for mock!")
        }

        fn clone_from(&mut self, _: &Self) {
            panic!("clone_from was attempted for mock!")
        }
    }

    #[tokio::test]
    async fn test() {
        let mut mock_my_trait = MockMyTrait::new();

        mock_my_trait.expect_foo().once().return_const(());

        let testable = ConcreteMiddleman::new(Box::new(mock_my_trait)).await;

        testable.do_middling_things();
    }
}
