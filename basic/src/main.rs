use std::ops::Deref;

struct DerefExample<T> {
    value: T
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
}
