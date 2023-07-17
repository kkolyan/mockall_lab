use std::cell::RefCell;
use mockall::automock;

#[automock]
pub trait MyTrait {
    fn do_something(&mut self);
}

pub struct MyStruct<T: MyTrait> {
    inner: T,
}

impl<T: MyTrait> MyStruct<T> {
    pub fn new(p0: T) -> Self {
        MyStruct { inner: p0 }
    }
}

impl<T: MyTrait> MyStruct<T> {
    pub fn do_something_root(&mut self) {
        self.inner.do_something();
    }
}

impl<T: MyTrait> MyTrait for &RefCell<T> {
    fn do_something(&mut self) {
        self.borrow_mut().do_something();
    }
}