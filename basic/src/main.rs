use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    let y = *x;
    let s = Deref::deref(&x);
    let z = *Deref::deref(&x);
    assert_eq!('a', *x);

    test_str();
}

fn test_str() {
    let s = 'a';
    let mut ss = String::new();

    for i in 0..10000 {
        let s_ = ((s as u8 + (i % 26) as u8) as char).to_string();
        ss += &s_;
        // s = (s as uint32 + i as uint32)
        println!("{}:{:?}", ss.len(), &ss);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
