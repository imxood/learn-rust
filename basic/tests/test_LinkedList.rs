#![feature(linked_list_cursors)]

use std::collections::LinkedList;

#[test]
fn test_linked_list() {
    let mut link: LinkedList<u8> = (0..10).collect();
    let mut cursor = link.cursor_back_mut();
    while let Some(v) = cursor.current() {
        if *v == 5 {
            cursor.insert_after(15);
        }
        cursor.move_prev();
    }
    println!("v: {:?}", &link);
}
