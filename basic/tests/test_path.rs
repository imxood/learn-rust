use std::path::Path;

#[test]
fn test_path() {
    let p = Path::new("a/b");
    println!("p: {:?}", p);

    let p1 = p.join("c");
    println!("p1: {:?}", &p1);

    let p = Path::new("a/b").join("/d");
    println!("p1: {:?}", &p);
}
