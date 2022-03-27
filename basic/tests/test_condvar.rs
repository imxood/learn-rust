use parking_lot::{Condvar, Mutex};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        loop {
            *lock.lock() = true;
            cvar.notify_one();

            std::thread::sleep(Duration::from_millis(1000));

            *lock.lock() = false;
            cvar.notify_one();

            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    loop {
        let mut started = lock.lock();
        println!("wait started...");
        cvar.wait(&mut started);
        println!("wait started finished, started: {}", *started);
    }
}

#[test]
fn test_condvar1() {
    #[derive(Clone, Default)]
    struct Runtime {
        pause: Arc<Mutex<bool>>,
        pause_cond: Arc<Condvar>,
    }

    impl Runtime {
        pub fn pause(&self) -> bool {
            *self.pause.lock()
        }

        pub fn set_pause(&self, pause: bool) {
            *self.pause.lock() = pause;
        }

        pub fn wait(&self) {
            self.pause_cond.wait(&mut self.pause.lock());
        }

        pub fn notify_all(&self) -> usize {
            self.pause_cond.notify_all()
        }
    }

    let runtime0 = Runtime::default();
    let runtime1 = runtime0.clone();

    thread::spawn(move || loop {
        if runtime0.pause() {
            println!("pause, wait nitofy...");
            runtime0.wait();
            println!("nitofy ok");
        } else {
            println!("running");
            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    loop {
        runtime1.set_pause(true);
        std::thread::sleep(Duration::from_millis(5000));

        runtime1.set_pause(false);
        runtime1.notify_all();
        std::thread::sleep(Duration::from_millis(5000));
    }
}

#[test]
fn test_condvar2() {
    #[derive(Clone, Default)]
    struct Runtime {
        pause: Arc<(Mutex<bool>, Condvar)>,
    }

    impl Runtime {
        pub fn pause(&self) -> bool {
            let (lock, _) = &*self.pause;
            *lock.lock()
        }

        pub fn set_pause(&self, pause: bool) {
            let (lock, _) = &*self.pause;
            *lock.lock() = pause;
        }

        pub fn wait(&self) {
            let (lock, cvar) = &*self.pause;
            cvar.wait(&mut lock.lock());
        }

        pub fn notify_all(&self) -> usize {
            let (_, cvar) = &*self.pause;
            cvar.notify_all()
        }
    }

    let runtime0 = Runtime::default();
    let runtime1 = runtime0.clone();

    thread::spawn(move || loop {
        if runtime0.pause() {
            println!("pause, wait nitofy...");
            runtime0.wait();
            println!("nitofy ok");
        } else {
            println!("running");
            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    loop {
        runtime1.set_pause(true);
        std::thread::sleep(Duration::from_millis(5000));

        runtime1.set_pause(false);
        runtime1.notify_all();
        std::thread::sleep(Duration::from_millis(5000));
    }
}
