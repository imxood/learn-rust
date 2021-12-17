#![allow(unused)]

#[macro_export]
macro_rules! flag {
    ($name:ident) => {
        #[derive(Default, Debug, Copy, Clone)]
        pub struct $name {
            bit: bool,
            single_bit: bool,
        }

        impl $name {
            pub fn get(&self) -> bool {
                self.bit
            }

            pub fn set(&mut self, set: bool) {
                self.bit = set;
            }

            pub fn single_get(&mut self) -> bool {
                let state = self.single_bit;
                self.single_bit = false;
                state
            }

            pub fn single_set(&mut self, set: bool) {
                self.single_bit = set;
            }
        }
    };
}

#[test]
fn test_proc1() {
    flag!(Stop);

    let mut stop_flag = Stop::default();
    println!("stop_flag: {:?}", &stop_flag);

    stop_flag.single_set(true);
    println!("stop_flag: {:?}", &stop_flag);

    let flag = stop_flag.single_get();
    println!("get flag: {:?}, stop_flag: {:?}, ", flag, &stop_flag);
}
