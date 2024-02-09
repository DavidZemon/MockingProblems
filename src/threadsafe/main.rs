use crate::dependency::MyTrait;
use crate::middleman::Middleman;
use dyn_clone::clone_box;

mod dependency;
mod middleman;

struct MyApp {
    my_trait: Box<dyn MyTrait>,
    middleman: Box<dyn Middleman>,
}

impl MyApp {
    fn new(my_trait: Box<dyn MyTrait>, middleman: Box<dyn Middleman>) -> Self {
        Self {
            my_trait,
            middleman,
        }
    }

    fn run(&self) {
        self.my_trait.foo();
        self.middleman.do_middling_things();
    }
}

#[tokio::main]
async fn main() {
    let my_struct = Box::new(dependency::MyStruct {});
    let middleman = Box::new(middleman::ConcreteMiddleman::new(clone_box(&*my_struct)).await);
    let my_app = MyApp::new(my_struct, clone_box(&*middleman));

    tokio::spawn(async move { middleman.do_middling_things() });
    my_app.run();
    println!("The app is done!");
}

#[cfg(test)]
mod test {
    use crate::dependency::MockMyTrait;
    use crate::middleman::MockMiddleman;
    use crate::MyApp;

    #[tokio::test]
    async fn test_happy_path() {
        let mut mock_my_trait = MockMyTrait::new();
        let mut mock_middleman = MockMiddleman::new();

        mock_my_trait.expect_foo().once().return_const(());
        mock_middleman
            .expect_do_middling_things()
            .once()
            .return_const(());

        let testable = MyApp::new(Box::new(mock_my_trait), Box::new(mock_middleman));

        testable.run();
    }
}
