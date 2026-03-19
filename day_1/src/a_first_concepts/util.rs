use std::cell::RefCell;
use std::rc::Rc;

pub(super) struct Droppable<T>(T, Rc<RefCell<Vec<T>>>) where T: Copy;

impl< T> Droppable< T> 
where T: Copy
{
    pub fn new(value: T, log: Rc<RefCell<Vec<T>>>) -> Self {
        Self(value, log)
    }
}

impl<T> Drop for Droppable<T>
where T: Copy
{
    fn drop(&mut self) {
        self.1.borrow_mut().push(self.0);
    }
}