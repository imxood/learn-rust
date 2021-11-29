use std::any::Any;

trait Animal {
    fn noise(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Sheep {}

impl Animal for Sheep {
    fn noise(&self) {
        println!("baaaaah!");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_dyn() {
    let sheep = Sheep {};
    let animate: &dyn Animal = &sheep;
    animate.noise();

    let sheep = unsafe { &*(animate.as_any() as *const dyn Any as *const Sheep) };
    sheep.noise();

    let sheep: &Sheep = animate
        .as_any()
        .downcast_ref::<Sheep>()
        .expect("Wasn't a trusty printer!");
    sheep.noise();
}

trait Shape {
    fn shape(&self) {
        println!("circle");
    }
}

impl Shape for Sheep {}
