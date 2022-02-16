use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[test]
fn test_fn() {
    #[derive(Debug)]
    struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    impl Drop for Color {
        fn drop(&mut self) {
            println!("Color dropped");
        }
    }

    pub type ShareCellType<T> = Rc<RefCell<T>>;

    pub struct ShareCell<T> {
        cell: ShareCellType<T>,
    }

    impl<T> ShareCell<T> {
        pub fn new(data: T) -> Self {
            Self {
                cell: Rc::new(RefCell::new(data)),
            }
        }
        pub fn as_mut(&self) -> RefMut<T> {
            self.cell.as_ref().borrow_mut()
        }

        pub fn as_ref(&self) -> Ref<T> {
            self.cell.as_ref().borrow()
        }
    }

    let cell = ShareCell::new(Color {
        r: 0xf1,
        g: 0xf1,
        b: 0xf1,
        a: 0xf1,
    });

    println!("color: {:x?}", cell.as_ref());

    // 调用后不释放, 也会 panic

    // let mut borrow_cell = cell.as_mut();

    // *borrow_cell = Color {
    //     r: 0xf2,
    //     g: 0xf2,
    //     b: 0xf2,
    //     a: 0xf2,
    // };

    // 多次调用 as_mut(), 会 panic

    *cell.as_mut() = Color {
        r: 0xf2,
        g: 0xf2,
        b: 0xf2,
        a: 0xf2,
    };

    println!("color: {:x?}", cell.as_ref());
}
