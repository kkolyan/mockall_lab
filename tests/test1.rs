use std::cell::RefCell;
use mockall_lab::{MockMyTrait, MyStruct};

#[test]
fn test1() {
    let mut mock = RefCell::new(MockMyTrait::new());

    let mut subject = MyStruct::new(&mock);

    {
        mock
            .borrow_mut()
            .expect_do_something()
            .once();
    }

    subject.do_something_root();


}