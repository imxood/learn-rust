use std::collections::LinkedList;

#[test]
fn test_mem_size() {
    let mut link = LinkedList::<A>::new();

    // LinkedList 内部数据主要是指针, 所以, 内存是固定的
    link.push_back(A::default());
    println!("link mem: {}", std::mem::size_of_val(&link));

    link.push_back(A::default());
    println!("link mem: {}", std::mem::size_of_val(&link));

    let mut vec0 = Vec::<A>::new();
    vec0.push(A::default());
    vec0.push(A::default());
    println!("vec mem: {}", std::mem::size_of_val(&vec0));

    println!("vec mem: {}", std::mem::size_of_val(&vec0));
}

#[derive(Default)]
#[repr(C)]
struct A {
    a0: usize,
    a1: usize,
}
