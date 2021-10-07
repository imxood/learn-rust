use std::{sync::Arc, thread::spawn};

use parking_lot::Mutex;

#[derive(Debug)]
struct Config {
    a: i32,
}

#[test]
fn test_mutex() {
    let a = Arc::new(Mutex::new(Config { a: 1 }));
    let a1 = a.clone();

    let task1 = spawn(move || {
        let a1 = a1.lock();
        let p_a1 = &*a1 as *const Config;
        println!("p_a1: {:?}", p_a1);
    });

    let task2 = spawn(move || {
        let a = a.lock();
        let p_a = &*a as *const Config;
        println!("p_a: {:?}", p_a);
    });

    task1.join().unwrap();
    task2.join().unwrap();
}

#[test]
fn test_parking_mutex() {
    let a = Arc::new(parking_lot::Mutex::new(Config { a: 1 }));
    let a1 = a.clone();

    let task1 = spawn(move || {
        let a1 = a1.lock();
        let p_a1 = &*a1 as *const Config;
        println!("p_a1: {:?}", p_a1);
    });

    let task2 = spawn(move || {
        let a = a.lock();
        let p_a = &*a as *const Config;
        println!("p_a: {:?}", p_a);
    });

    task1.join().unwrap();
    task2.join().unwrap();
}

#[test]
fn test_mutex_lock() {
    #[derive(Debug, Clone, Copy)]
    enum Status {
        Running,
        Stopped,
    }

    struct ObjA {
        pub status: Status,
    }

    impl ObjA {
        pub fn echo(&self) {
            println!("echo: {:?}", &self.status);
        }
        pub fn get(&self) -> Status {
            self.status.clone()
        }
    }

    struct Obj {
        pub a: Arc<std::sync::Mutex<ObjA>>,
    }

    let obj = Obj {
        a: Arc::new(std::sync::Mutex::new(ObjA {
            status: Status::Stopped,
        })),
    };

    match obj.a.lock().unwrap().get() {
        Status::Running => {
            obj.a.lock().unwrap().echo();
        }
        Status::Stopped => {
            obj.a.lock().unwrap().echo();
        }
    };

    // let status = obj.a.lock().unwrap().get();
    // match status {
    //     Status::Running => {
    //         obj.a.lock().unwrap().echo();
    //     }
    //     Status::Stopped => {
    //         obj.a.lock().unwrap().echo();
    //     }
    // };
}

#[test]
fn test_arc() {
    #[derive(Debug, Clone, Copy)]
    enum Status {
        Running,
        Stopped,
    }

    struct ObjA {
        pub status: Status,
    }

    impl ObjA {
        pub fn echo(&self) {
            println!("echo: {:?}", &self.status);
        }
        pub fn get(&self) -> Status {
            self.status.clone()
        }
    }

    impl Drop for ObjA {
        fn drop(&mut self) {
            println!("ObjA dropped");
        }
    }

    struct SharedData {
        pub a: Arc<std::sync::Mutex<ObjA>>,
    }

    impl Drop for SharedData {
        fn drop(&mut self) {
            println!("SharedData dropped");
        }
    }

    struct Obj {
        pub sd: SharedData,
    }

    impl Drop for Obj {
        fn drop(&mut self) {
            println!("Obj dropped");
        }
    }

    let obj = Obj {
        sd: SharedData {
            a: Arc::new(std::sync::Mutex::new(ObjA {
                status: Status::Stopped,
            })),
        },
    };

    let obj_a = obj.sd.a.clone();
    drop(obj);

    obj_a.lock().unwrap().echo();
}
