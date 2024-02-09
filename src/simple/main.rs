use crate::dependency::MyTrait;

mod dependency;

struct MyApp<'a> {
    my_trait: &'a dyn MyTrait,
}

impl<'a> MyApp<'a> {
    fn new(my_trait: &'a dyn MyTrait) -> Self {
        Self { my_trait }
    }

    fn run(&self) {
        self.my_trait.foo()
    }
}

fn main() {
    let my_struct = dependency::MyStruct {};
    let my_app = MyApp::new(&my_struct);
    my_app.run();
    println!("The app is done!");
}

#[cfg(test)]
mod test {
    use crate::dependency::MockMyTrait;
    use crate::MyApp;

    #[test]
    fn test_happy_path() {
        let mut mock_my_trait = MockMyTrait::new();

        mock_my_trait.expect_foo().once().return_const(());

        let testable = MyApp::new(&mock_my_trait);

        testable.run();
    }
}
