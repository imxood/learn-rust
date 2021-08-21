use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
struct Data {
    pub a: Rc<RefCell<Option<i32>>>,
    pub b: Rc<RefCell<Option<i32>>>,
}

#[test]
fn test_refcell() {
    let d = Data::default();
    let mut d_a = d.a.borrow_mut().clone();
    d_a = None;
    println!("d_a: {:?}", &d.a);
    let d_a1 = d_a.clone();
    let count = Rc::strong_count(&d.a);
    println!("count: {}", count);
}
